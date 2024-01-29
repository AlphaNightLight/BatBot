use std::{
    alloc::{alloc_zeroed, dealloc, handle_alloc_error, Layout},
    ffi::c_void,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::{
    generated::{self, new_serial_hal},
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
            let t = data as *mut S;
            //we don't want to deallocate what it was previously here
            t.write(s);
            let mut serial_hal = new_serial_hal();
            serial_hal.init1(data as *mut c_void,
                Some(S::unsafe_available),
                Some(S::unsafe_send),
                Some(S::unsafe_read),
                Some(S::unsafe_flush),);
            p.init(
                serial_hal
            );
            Self { p, ph: PhantomData }
        }
    }
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
        let ptr = self.p.checker.serial.data as *mut u8;
        unsafe {
            dealloc(ptr, layout);
        }
    }
}
