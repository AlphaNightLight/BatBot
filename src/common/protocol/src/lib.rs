/// this should never be pub
mod generated;

pub mod protocol;
pub mod serial;
pub use protocol::Protocol;

#[cfg(test)]
mod test {

    use crate::{
        serial::{Serial, TestSerial},
        Protocol,
    };

    #[test]
    fn test_protocol_init() {
        let (robot, mut t) = TestSerial::new(0.0);
        let mut p = Protocol::new(robot);
        //send
        unsafe {
            let mut to_send: Vec<u8> = vec![0, 10];
            let len = to_send.len() as u8;
            let buff = to_send.as_mut_ptr();
            p.checker.send_msg(buff, len);
        }
        let len = t.available();
        let mut buff = Vec::new();
        for _ in 0..len {
            buff.push(t.read());
        }
        assert_eq!(vec![35, 0, 10, 30, 2, 69], buff);
        //SEND SUCCESS!!!

        //panic!("{}", t.read())
    }
}

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
