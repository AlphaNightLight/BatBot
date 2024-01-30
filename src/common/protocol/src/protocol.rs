use std::{
    alloc::{alloc_zeroed, dealloc, handle_alloc_error, Layout}, ffi::c_void, marker::PhantomData, mem, ops::{Deref, DerefMut}, slice
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
            serial_hal.init1(
                data as *mut c_void,
                Some(S::unsafe_available),
                Some(S::unsafe_send),
                Some(S::unsafe_read),
                Some(S::unsafe_flush),
            );
            p.init(serial_hal);
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

impl<S: Serial> Protocol<S>{
    /// tries to read from Protocol
    /// It could return nothing, if expected to read something but didn't received anything for a while consider reconnecting
    pub fn read(&mut self) -> Option<Vec<u8>> {
        //print!("t");
        unsafe {
            if self.checker.try_read_message() {
                let len = self.checker.out_len as usize;
                let v = self.checker.out_buffer.iter().take(len).copied().collect();

                self.checker.out_buffer.iter_mut().for_each(|m| *m = 0);
                return Some(v);
            }
        }
        None
    }

    ///tries to send msg over serial, if possible
    pub fn send(&mut self, to_send: &mut [u8]) {
        unsafe {
            let len = to_send.len() as u8;
            let buff = to_send.as_mut_ptr();
            self.checker.send_msg(buff, len);
        }
    }
    ///tries to send a whole struct over serial, if possible
    pub fn send_struct<T>(&mut self, mut to_send: T){
        let config_size = mem::size_of::<T>();
        unsafe{
            let u8_slice = slice::from_raw_parts_mut(&mut to_send as *mut _ as *mut u8, config_size);
            self.send(u8_slice);
        }
    }
}