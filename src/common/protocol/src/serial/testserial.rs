use std::{
    borrow::BorrowMut,
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use rand::{rngs::OsRng, Rng};

use super::Serial;

/// Serial used for testing purpose:
///
/// it's needed to specify the error rate at creation time
pub struct TestSerial {
    pub other_rx: Arc<Mutex<VecDeque<u8>>>,
    pub rx: Arc<Mutex<VecDeque<u8>>>,
    error_rate: f64,
}

impl TestSerial {
    ///Create a new TestSerial channel with the specified error rate
    pub fn new(error_rate: f64) -> (Self, Self) {
        let tmp = Arc::new(Mutex::new(VecDeque::new()));
        let mut a = Self {
            rx: Arc::new(Mutex::new(VecDeque::new())),
            other_rx: tmp,
            error_rate,
        };
        let b = Self {
            rx: Arc::new(Mutex::new(VecDeque::new())),
            other_rx: a.rx.clone(),
            error_rate,
        };
        a.other_rx = b.rx.clone();
        (a, b)
    }
}

impl Serial for TestSerial {
    fn send(&mut self, d: u8) {
        let to_send = if OsRng.gen_bool(self.error_rate) {
            OsRng.gen()
        } else {
            d
        };

        self.other_rx
            .lock()
            .unwrap()
            .borrow_mut()
            .push_back(to_send);
    }

    fn read(&mut self) -> u8 {
        self.rx.lock().unwrap().pop_front().unwrap_or(0)
    }

    fn available(&mut self) -> i32 {
        self.rx.lock().unwrap().len() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::{super::Serial, TestSerial};

    #[test]
    fn test_test_serial() {
        let (mut a, mut b) = TestSerial::new(0.0);
        a.send(b'0');
        assert_eq!(b.read(), b'0');
        b.send(b'0');
        assert_eq!(a.read(), b'0');
    }
}
