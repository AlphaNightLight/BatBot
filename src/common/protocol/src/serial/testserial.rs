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

    fn flush(&mut self) {
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

#[cfg(test)]
mod test {

    use rand::{thread_rng, Rng};
    use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

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
            p.send_msg(buff, len);
        }
        let len = t.available();
        let mut buff = Vec::new();
        for _ in 0..len {
            buff.push(t.read());
        }
        assert_eq!(vec![35, 0, 10, 30, 2, 69], buff);
        //SEND SUCCESS!!!
    }

    fn send<S: Serial>(robot: &mut Protocol<S>, to_send: &mut [u8]) {
        unsafe {
            let len = to_send.len() as u8;
            let buff = to_send.as_mut_ptr();
            robot.send_msg(buff, len);
        }
    }

    fn read<S: Serial>(pc: &mut Protocol<S>) -> Option<Vec<u8>> {
        unsafe {
            if pc.try_read_message() {
                let v = pc.out_buffer.to_vec();
                pc.out_buffer.iter_mut().for_each(|m| *m = 0);
                Some(v)
            } else {
                None
            }
        }
    }
    fn prob_wrong(e: f64, n: i32) -> f64 {
        let e = e * 255. / 256.;
        let n = n + 1;
        let mut ret = 0.;

        for i in 1..=n {
            let cur =
                (e).powi(i) * (1. - e).powi(n - i + 3) * (fact(n) / fact(i) / fact(n - i)) as f64;
            ret += cur * prob_autocompensating(i);
        }
        //completly random reusing start;
        ret += (1. / 256.0f64).powi(3) * (n as f64 - 3.); //TODO when OUT_BUFFER_SIZE>n si possono riciclare più start (errori  aumentano)
        ret
    }
    fn prob_autocompensating(n_errori: i32) -> f64 {
        let mut a = 1. / 255.;
        let mut b = 0.;

        for _ in 1..n_errori {
            let na = (254. * a + b) / 255.;
            let nb = a;
            a = na;
            b = nb;
        }
        b
    }

    fn fact(n: i32) -> i32 {
        if n == 0 {
            1
        } else {
            fact(n - 1) * n
        }
    }

    fn test_distribution(e: f64, length: i32, n: i32) -> Result<(), String> {
        let (robot, pc) = TestSerial::new(e);
        let mut robot = Protocol::new(robot);
        let mut pc = Protocol::new(pc);
        let mut true_positive = 0;
        let mut true_negative = 0;
        let mut false_positive = 0;
        // false negative should be impossible...

        'outher: for _ in 0..n {
            let mut rng = thread_rng();
            let mut to_send: Vec<u8> = (0..length).map(|_| rng.gen()).collect();

            send(&mut robot, &mut to_send);
            while let Some(v) = read(&mut pc) {
                if v.iter().zip(to_send.iter()).all(|(a, b)| a == b) {
                    true_positive += 1;
                    continue 'outher;
                } else {
                    false_positive += 1;
                }
            }
            true_negative += 1;
        }
        println!("true_positive={true_positive} \n true_negative={true_negative}  false_positive = {false_positive}");
        let prob_resend = 1. - (1. - e * 255. / 256.).powf(length as f64 + 4.);
        let sigma = prob_resend * (1. - prob_resend);
        let sigma = (sigma * n as f64).sqrt();
        //println!("{:.6}\t{:.6}\t{}", 1.-prob_resend, prob_resend, sigma);
        let min = (prob_resend * n as f64 - sigma * 3.29).floor() as i32 - 1;
        let max = (prob_resend * n as f64 + sigma * 3.29).ceil() as i32 + 1;
        //
        println!(
            "{:.0} < {true_negative} < {:.0} for 99.9% of times (confidence level 3.29s)",
            min, max
        );
        if true_negative < min || max < true_negative {
            return Err(format!("wrong distribution, it may be a fluke( using 0.9999 confidence level) n={n} error_rate={e}, {min:.0}<{true_negative}<{max:.0}"));
        }
        //let prob_fp = (1.-(1.-e).powf(11.))/256.;
        let prob_fp = prob_wrong(e, length);
        let sigma = prob_fp * (1. - prob_fp);
        let sigma = (sigma * n as f64).sqrt();

        let min = (prob_fp * n as f64 - sigma * 3.29).floor() as i32 - 1;
        let max = (prob_fp * n as f64 + sigma * 3.29).ceil() as i32 + 1;
        //
        println!(
            "{:.0} < {false_positive} < {:.0} for more of 99.9% of times (confidence level 3.29s {} {})",
            min,
            max,
            sigma,
            prob_fp * n as f64
        );
        if false_positive < min || max < false_positive {
            return Err(format!("wrong distribution, it may be a fluke( using 0.9999 confidence level) n={n} error_rate={e}, {min:.0}<{true_negative}<{max:.0}"));
        }
        Ok(())
    }

    #[test]
    fn test_protocol() {
        let mut to_exec = Vec::<(f64, i32, i32)>::new();
        for n in (3..=6).rev().map(|x| 10.0f32.powi(x) as i32) {
            for e in (1..=6).map(|x| 0.1f64.powi(x)) {
                for l in 1..=10 {
                    to_exec.push((e, l, n));
                }
            }
        }
        let res: Result<(), String> = to_exec
            .par_iter()
            .map(|(e, l, n)| test_distribution(*e, *l, *n))
            .collect();
        println!("{:?}", res);
    }
}
