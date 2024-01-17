
use std::{time:: Instant, error::Error};

use bluer::{rfcomm::{Socket, SocketAddr, Stream}, Address};
use rand::{rngs::OsRng, Rng};
use tokio::{runtime::Runtime, io::AsyncWriteExt, io::AsyncReadExt};

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
        //let session = bluer::Session::new().await.unwrap();
        //let adapter = session.default_adapter().await.unwrap();
        //adapter.set_powered(false).await.unwrap();
       // adapter.set_powered(true).await.unwrap();
        //sleep(Duration::from_millis(1000));
        
        println!("out_buffer= {}", s.input_buffer().unwrap());
        let mut t = connect().await.unwrap();
        //pin_mut!(t);
        //println!("{:?}", t);
        let mut buf =[0u8; 1000];
        let start = Instant::now();
        let mut tot_sent=0;
        loop{
            let to_send: Vec<u8> = (0..100).map(|_| OsRng.gen::<u8>()).collect(); 
            t.write_all(to_send.as_slice()).await.unwrap();
            t.flush().await.unwrap();

            /*let received = t.peek(&mut buf).await.unwrap();
            while received<10{
                let received = t.peek(&mut buf).await.unwrap();
                println!("{}", received);
            }*/
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
