use std::{
    alloc::{alloc_zeroed, dealloc, Layout},
    ffi::c_void,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    ptr,
};

use crate::{
    generated,
    serial::{CSerial, Serial},
};

/// c Protocol wrapper
pub struct Protocol<S: Serial> {
    ph: PhantomData<S>,
    p: generated::Protocol,
}

impl<S: Serial> Protocol<S> {
    /// get a freshly new protocol out of this
    pub fn new(mut s: S) -> Self {
        let mut p = unsafe { generated::new_protocol() };

        unsafe {
            let layout = Layout::new::<S>();
            let data = alloc_zeroed(layout);
            let t = &mut s as *mut S;
            ptr::copy(t as *mut u8, data, core::mem::size_of::<S>());
            *t = s;
            p.init(
                data as *mut c_void,
                Some(S::unsafe_available),
                Some(S::unsafe_send),
                Some(S::unsafe_read),
            );
            Self { p, ph: PhantomData }
        }
    }

    /*pub fn inner(&mut self) -> &mut generated::Protocol {
        &mut self.p
    }*/
}

impl<S: Serial> Deref for Protocol<S> {
    type Target = generated::Protocol;

    fn deref(&self) -> &Self::Target {
        &self.p
    }
}
impl<S: Serial> DerefMut for Protocol<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.p
    }
}

impl<S: Serial> Drop for Protocol<S> {
    fn drop(&mut self) {
        let layout = Layout::new::<S>();
        let ptr = self.p.checker.data as *mut u8;
        unsafe {
            dealloc(ptr, layout);
        }
    }
}
