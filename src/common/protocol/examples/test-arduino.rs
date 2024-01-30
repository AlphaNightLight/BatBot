use std::{thread::sleep, time::{Duration, Instant}};

use bluer::{Address, Uuid};
use protocol::{
    serial::Ble,
    Protocol,
};
use tokio::runtime::Runtime;

fn main() {
    let runtime = Runtime::new().unwrap();
    /*let bl = Bluetooth::try_new(
        Address::new([0x00, 0x18, 0x91, 0xD8, 0xE9, 0xC7]),
        runtime.handle().clone(),
    )
    .unwrap();*/
    
    let bl = Ble::try_new(
        Address::new([0xA8, 0x10, 0x87, 0x67, 0x73, 0x2A]),
        Uuid::from_bytes([
            0x00, 0x00, 0xFF, 0xE0, 0x00, 0x00, 0x10, 0x00, 0x80, 0x00, 0x00, 0x80, 0x5F, 0x9B, 0x34, 0xFB,
        ]),
        runtime.handle().clone(),
    )
    .unwrap();
    let mut protocol = Protocol::new(bl);
    println!("connesso, in attesa");
    let mut prev = 0;
    let mut t = None;
    let mut nerror = 0;
    let mut correct = 0;
    let mut wrong_wave=0;
    loop {
        //println!("loop");
        if let Some(x) = protocol.read().and_then(|x| {
            //println!("some {:?}", x);
                String::from_utf8(x).ok()
            }) {
            wrong_wave=0;
            if t.is_none() {
                t = Some(Instant::now());
            }
            //println!("{} {:?} {}", x, x,x.len());
            let numero = x[9..].parse::<i32>().unwrap();
            if numero != prev + 1 {
                nerror += 1;
            } else {
                correct += 1;
                wrong_wave=0;
            }
            prev = numero;
            let msgs = (correct + nerror) as f32 / t.unwrap().elapsed().as_secs_f32();

            println!(
                "{} {} {:.3} {:.0}",
                correct,
                nerror,
                msgs,
                msgs * (x.len() + 4) as f32 * 10.
            );
        }else{
            sleep(Duration::from_millis(1));
            wrong_wave+=1;
            if wrong_wave>1000{
                if let Ok(bl) = Ble::try_new(
                    Address::new([0xA8, 0x10, 0x87, 0x67, 0x73, 0x2A]),
                    Uuid::from_bytes([
                        0x00, 0x00, 0xFF, 0xE0, 0x00, 0x00, 0x10, 0x00, 0x80, 0x00, 0x00, 0x80, 0x5F, 0x9B, 0x34, 0xFB,
                    ]),
                    runtime.handle().clone(),
                ){
                    protocol=Protocol::new(bl);
                    t=None;
                    wrong_wave=0;
                    correct=0;
                    nerror=0;
                }
                
            }
        }
    }
}