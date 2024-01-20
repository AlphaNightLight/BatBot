use std::time::Instant;

use bluer::Address;
use rand::{rngs::OsRng, Rng};
use tokio::runtime::Runtime;
/*
async fn connect()->Result<Stream, Box<dyn Error>>{
    for i in 0..10{
        println!("tentativo {i}");
        let s = Socket::new().unwrap();
        s.set_recv_buffer(100).unwrap();
        if let Ok(s) =s.connect(SocketAddr::new(Address::new([0x00, 0x18, 0x91, 0xD8, 0xE9, 0xC7]), 1)).await{
            return Ok(s);
        }
    }
    todo!()

}

fn main() {
    let rt = Runtime::new().unwrap();
    let _guard = rt.enter();
    let s = Socket::new().unwrap();

    rt.block_on(async {

        println!("out_buffer= {}", s.input_buffer().unwrap());
        let mut t = connect().await.unwrap();
        //pin_mut!(t);
        //println!("{:?}", t);
        let mut buf =[0u8; 1000];
        let start = Instant::now();
        let mut tot_sent=0;
        loop{
            let to_send: Vec<u8> = (0..16).map(|_| OsRng.gen::<u8>()).collect();
            t.write_all(to_send.as_slice()).await.unwrap();
            t.flush().await.unwrap();
            let received = t.read(&mut buf).await.unwrap();
            //println!("{:?}->{}", to_send, received);
            tot_sent+=received;
            let elapsed = start.elapsed().as_secs_f32();
            println!("{:.0} {} {}", 10.*tot_sent as f32/elapsed, tot_sent, elapsed);
            //time::sleep(Duration::from_millis(1000)).await;
        }

    });

    println!("{:?}", s.conn_info());
}
*/
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
    let length = 40;
    let mut corretti = 0;
    let mut falsi_positivi = 0;
    let mut totali = 0;
    let start = Instant::now();
    loop {
        let mut to_send: Vec<u8> = (0..length).map(|_| OsRng.gen()).collect();
        //println!("sent");
        send(&mut protocol, &mut to_send);
        //corretti+=1;

        for _ in 0..10 {
            //println!("tentativo");
            if let Some(readen) = read(&mut protocol) {
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
    }
    //}
    None
}
