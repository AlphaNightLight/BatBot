use std::error::Error;

use bluer::{
    rfcomm::{Socket, SocketAddr, Stream},
    Address,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    runtime::Handle,
};

use super::Serial;

///Address::new([0x00, 0x18, 0x91, 0xD8, 0xE9, 0xC7])
pub struct Bluetooth {
    to_send: Vec<u8>,
    handle: Handle,
    stream: Stream,
}
impl Bluetooth {
    pub fn try_new(address: Address, handle: Handle) -> Result<Self, Box<dyn Error>> {
        let stream = handle.block_on(async { connect(address).await })?;
        let mut ret = Self {
            to_send: Vec::new(),
            handle,
            stream: stream,
        };
        ret.send(0);
        Ok(ret)
    }
}

async fn connect(address: Address) -> Result<Stream, Box<dyn Error>> {
    for i in 0..10 {
        println!("tentativo {i}");
        let s = Socket::new().unwrap();
        s.set_recv_buffer(100).unwrap();

        if let Ok(mut s) = s.connect(SocketAddr::new(address, 1)).await {
            s.write_u8(0).await.unwrap();
            return Ok(s);
        }
    }
    Err("can't connect, the device haven't respond 10 times in a row")?
}

impl Serial for Bluetooth {
    fn flush(&mut self) {
        self.handle.block_on(async {
            self.stream.write_all(&self.to_send).await.unwrap();
            self.to_send.clear();
            self.stream.flush().await.unwrap()
        });
    }

    fn send(&mut self, d: u8) {
        self.to_send.push(d);
    }

    fn read(&mut self) -> u8 {
        self.handle
            .block_on(async { self.stream.read_u8().await.unwrap() })
    }

    fn available(&mut self) -> i32 {
        let ret = self.handle.block_on(async {
            let mut buf = [0u8; 100];
            self.stream.peek(&mut buf).await.unwrap() as i32
        });
        ret
    }
}
