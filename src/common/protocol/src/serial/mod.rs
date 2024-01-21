#[cfg(feature = "rfcomm")]
mod bluetooth;
#[cfg(feature = "rfcomm")]
pub use bluetooth::Bluetooth;

#[cfg(feature = "test-serial")]
mod testserial;
#[cfg(feature = "test-serial")]
pub use testserial::TestSerial;

#[cfg(feature = "ble")]
mod bluetooth_low_energy;
#[cfg(feature = "ble")]
pub use bluetooth_low_energy::Ble;

use std::ffi::c_void;

///Serial interface
///
/// It's an abstraction on how a Serial interface should work

pub trait Serial {
    fn flush(&mut self);
    fn send(&mut self, d: u8);
    /// this could fail, but the c impl doesn't need to know that
    fn read(&mut self) -> u8;
    fn available(&mut self) -> i32;
}

/// This trait is the C friendly Serial interface
///
/// It get's implemented automagicaly for every struct that implements serial
///
pub trait CSerial: Serial {
    /// C wrapper around Serial::available
    ///
    /// # Safety
    ///
    /// this parameter must be a valid pointer to the implemented struct
    unsafe extern "C" fn unsafe_available(this: *mut c_void) -> i32;

    /// C wrapper around Serial::send
    ///
    /// # Safety
    ///
    /// this parameter must be a valid pointer to the implemented struct
    unsafe extern "C" fn unsafe_send(this: *mut c_void, d: u8);

    /// C wrapper around Serial::read
    ///
    /// # Safety
    ///
    /// this parameter must be a valid pointer to the implemented struct
    unsafe extern "C" fn unsafe_read(this: *mut c_void) -> u8;

    /// C wrapper around Serial::flush
    ///
    /// # Safety
    ///
    /// this parameter must be a valid pointer to the implemented struct
    unsafe extern "C" fn unsafe_flush(this: *mut c_void);
}

impl<T: Serial> CSerial for T {
    unsafe extern "C" fn unsafe_available(this: *mut c_void) -> i32 {
        let x = this as *mut T;
        T::available(&mut *x)
    }

    unsafe extern "C" fn unsafe_send(this: *mut c_void, d: u8) {
        let x = this as *mut T;
        T::send(&mut *x, d)
    }

    unsafe extern "C" fn unsafe_read(this: *mut c_void) -> u8 {
        let x = this as *mut T;
        T::read(&mut *x)
    }
    unsafe extern "C" fn unsafe_flush(this: *mut c_void) {
        let x = this as *mut T;
        T::flush(&mut *x);
    }
}
