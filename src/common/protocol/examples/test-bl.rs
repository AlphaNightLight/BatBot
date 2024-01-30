use std::time::Instant;

use bluer::Address;
use rand::{rngs::OsRng, Rng};
use tokio::runtime::Runtime;

use protocol::{
    serial::{Bluetooth, Serial},
    Protocol,
};
fn main() {
    let runtime = Runtime::new().unwrap();
    let mut bl = Bluetooth::try_new(
        Address::new([0x00, 0x18, 0x91, 0xD8, 0xE9, 0xC7]),
        runtime.handle().clone(),
    )
    .unwrap();
    bl.send(0);
    let mut protocol = Protocol::new(bl);
    let length = 194;
    let mut corretti = 0;
    let mut falsi_positivi = 0;
    let mut totali = 0;
    let start = Instant::now();
    loop {
        let mut to_send: Vec<u8> = (0..length).map(|_| OsRng.gen()).collect();
        //println!("sent");
        protocol.send( &mut to_send);
        //corretti+=1;

        for _ in 0..10 {
            //println!("tentativo");
            if let Some(readen) = protocol.read() {
                if to_send.iter().zip(readen.iter()).all(|(x, y)| *x == *y) {
                    corretti += 1;
                    break;
                } else {
                    falsi_positivi += 1;
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
