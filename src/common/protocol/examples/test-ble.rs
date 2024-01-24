use std::{
    error::Error,
    thread::sleep,
    time::{Duration, Instant},
};

use bluer::{Address, Uuid};
use rand::{rngs::OsRng, Rng};
use tokio::runtime::{Handle, Runtime};

use protocol::{
    serial::{Ble, Serial},
    Protocol,
};
fn new_protocol(handle: Handle) -> Result<Protocol<Ble>, Box<dyn Error>> {
    let bl = Ble::try_new(
        //A8:10:87:67:73:2A
        Address::new([0xA8, 0x10, 0x87, 0x67, 0x73, 0x2A]),
        //0000ffe0-0000-1000-8000-00805f9b34fb
        Uuid::from_bytes([
            0x00, 0x00, 0xFF, 0xE0, 0x00, 0x00, 0x10, 0x00, 0x80, 0x00, 0x00, 0x80, 0x5F, 0x9B,
            0x34, 0xFB,
        ]),
        handle,
    )?;
    let protocol: Protocol<Ble> = Protocol::new(bl);
    Ok(protocol)
}
fn main() {
    let runtime = Runtime::new().unwrap();
    let mut protocol = new_protocol(runtime.handle().clone()).unwrap();
    let length = 194;
    let mut corretti = 0;
    let mut falsi_positivi = 0;
    let mut totali = 0;
    let start = Instant::now();
    let mut invalid=0;
    loop {
        let mut to_send: Vec<u8> = (0..length).map(|_| OsRng.gen()).collect();
        //println!("sent");
        send(&mut protocol, &mut to_send);
        //corretti+=1;
        let l= 2000;
        for i in 0..l {
            //println!("tentativo");
            if let Some(readen) = read(&mut protocol) {
                invalid=0;
                if to_send.iter().zip(readen.iter()).all(|(x, y)| *x == *y) {
                    corretti += 1;
                    
                    break;
                } else {
                    falsi_positivi += 1;
                }
            }
            sleep(Duration::from_millis(1));
            if i == l-1 {
                invalid+=1;
                if invalid<3{
                    continue;
                }
                if let Ok(x) = new_protocol(runtime.handle().clone()) {
                    protocol = x;
                    invalid=0;
                }
                
            }
        }

        totali += 1;
        println!(
            "{corretti} {falsi_positivi} {totali} {} {:.0} baud used {:.2} msg/s",
            corretti as f32 / totali as f32,
            ((length + 4) * corretti * 10) as f32 / start.elapsed().as_secs_f32(),
            corretti as f32 / start.elapsed().as_secs_f32()
        );
        //println!("{:?} {:?}", to_send, &readen[0..length]);
    }
}
fn send<S: Serial>(robot: &mut Protocol<S>, to_send: &mut [u8]) {
    unsafe {
        let len = to_send.len() as u8;
        let buff = to_send.as_mut_ptr();
        robot.checker.send_msg(buff, len);
    }
}

fn read<S: Serial>(pc: &mut Protocol<S>) -> Option<Vec<u8>> {
    //for _ in 0..20{
    unsafe {
        while pc.checker.try_read_message() {
            let v = pc.checker.out_buffer.to_vec();
            pc.checker.out_buffer.iter_mut().for_each(|m| *m = 0);
            return Some(v);
        }
        //sleep(Duration::from_millis(10));
    }
    //}
    None
}
