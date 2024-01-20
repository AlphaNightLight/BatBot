/// this should never be pub
mod generated;

pub mod protocol;
pub mod serial;
pub use protocol::Protocol;

/*
#[cfg(test)]
mod test {
    use std::{cell::OnceCell, os::raw::c_void};

    use crate::{
        serial::{CSerial, Serial, TestSerial},
        Protocol,
    };

    fn init() {
        let (mut x, mut t) = TestSerial::new(0.0);
        let robot = Protocol {
            data: (&mut x as *mut TestSerial) as *mut c_void, // casting from rust pointer to c_pointer
            available: Some(TestSerial::unsafe_available),
            send: Some(TestSerial::unsafe_send),
            read: Some(TestSerial::unsafe_read),
        };
    }
}*/
