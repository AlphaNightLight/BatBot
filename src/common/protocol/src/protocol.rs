use std::{
    alloc::{alloc_zeroed, dealloc, Layout, handle_alloc_error},
    ffi::c_void,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    ptr::{self, addr_of_mut}, mem::size_of,
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
    pub fn new(s: S) -> Self {
        let mut p = unsafe { generated::new_protocol() };

        unsafe {
            let layout = Layout::new::<S>();
            let data = alloc_zeroed(layout);
            if data.is_null() {
                handle_alloc_error(layout);
            }
            /*let t = &mut s as *mut S;
            ptr::copy(t as *mut u8, data, core::mem::size_of::<S>());
            *t = s;*/
            //let t = &mut s as *mut S;
            let t = data as *mut S;
            
            //addr_of_mut!(t).write(&s);
            //println!("coping {} bytes {:p}->{:p} {:p}", size_of::<S>(),&s, data, t);
            t.write(s);
           // *(t) = s;
            //println!("copied");
            //*t=s;
            p.init(
                data as *mut c_void,
                Some(S::unsafe_available),
                Some(S::unsafe_send),
                Some(S::unsafe_read),
                Some(S::unsafe_flush),
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
