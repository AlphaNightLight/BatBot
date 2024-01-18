use std::error::Error;

use bluer::{rfcomm::{Socket, SocketAddr, Stream}, Address};
use tokio::{runtime::Handle, io::{AsyncWriteExt, AsyncReadExt}};

use super::Serial;

///Address::new([0x00, 0x18, 0x91, 0xD8, 0xE9, 0xC7])
pub struct Bluetooth{
    //runtime: Runtime,
    to_send: Vec<u8>,
    handle: Handle,
    stream: Stream,
}
impl Bluetooth{
    pub fn try_new(address: Address, handle: Handle)->Result<Self, Box<dyn Error>>{
        //let runtime = Runtime::new().unwrap();
        let stream = handle.block_on(async{
            connect(address).await
        })?;
        let mut ret = Self{
            to_send: Vec::new(),
            handle,
            stream: stream,
        };
        ret.send(0);
        Ok(ret)
    }
}

async fn connect(address: Address)->Result<Stream, Box<dyn Error>>{
    for i in 0..10{
        println!("tentativo {i}");
        let s = Socket::new().unwrap();
        s.set_recv_buffer(100).unwrap();

        if let Ok(mut s) =s.connect(SocketAddr::new(address, 1)).await{
            s.write_u8(0).await.unwrap();
            return Ok(s);
        }
    }
    todo!()
    
}
/*
pub fn main() {
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
}*/


impl Serial for Bluetooth{
    fn flush(&mut self) {
        self.handle.block_on(async {
            self.stream.write_all(&self.to_send).await.unwrap();
            self.to_send.clear();
            self.stream.flush().await.unwrap()
            
        });
    }

    fn send(&mut self, d: u8) {
        self.to_send.push(d);
        //println!("{:p}", self);
        /*self.handle.block_on(async {
            self.stream.write_u8(d).await.unwrap();
        });*/
    }

    fn read(&mut self) -> u8 {
        self.handle.block_on(async {
            self.stream.read_u8().await.unwrap()
        })
    }

    fn available(&mut self) -> i32 {
        self.handle.block_on(async {
            let mut buf = [0u8; 100];
            self.stream.peek(&mut buf).await.unwrap() as i32
        })
    }
}