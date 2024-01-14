use std::{error::Error, sync::mpsc::{self, Sender, Receiver}};

use bluer::{AdapterEvent, Adapter};
use futures::{pin_mut, StreamExt, Stream, executor::block_on};
use tokio::{runtime::Runtime, task::spawn_blocking};

use super::Serial;


struct InnerBluetooth{
    to_send: mpsc::Receiver<u8>,
    readen: mpsc::Sender<u8>,
}

impl InnerBluetooth{
    fn new(to_send: Receiver<u8>, readen: Sender<u8>)->Self{
        Self {
            to_send,
            readen
        }
    }
    pub async fn discover(&self, adapter: Adapter)->Result<(), Box<dyn Error+Send>>{
        let discover = adapter.discover_devices().await?;
        // pin discover (not going out of scope in async)
        pin_mut!(discover);
        while let Some(evt) = discover.next().await{
            match evt{
                AdapterEvent::DeviceAdded(_) => todo!(),
                AdapterEvent::DeviceRemoved(_) => todo!(),
                AdapterEvent::PropertyChanged(_) => todo!(),
            }
        }
        todo!()
    }
}

pub struct Bluetooth{
    runtime: Runtime,
    inner: InnerBluetooth,
    tx: mpsc::Receiver<u8>,
    rx: mpsc::Sender<u8>,
}

impl Bluetooth{
    pub fn new()->Result<Self, Box<dyn Error>>{
        // creating async runtime

        let rt = Runtime::new().unwrap();
        let _guard = rt.enter();
        let adapter: Adapter= rt.block_on(async{
            //getting session
            let session = bluer::Session::new().await?;
            let adapter = session.default_adapter().await?;

            //reboot adapter
            adapter.set_powered(false).await?;
            adapter.set_powered(true).await?;

            println!(
                "Discovering on Bluetooth adapter {} with address {}\n",
                adapter.name(),
                adapter.address().await?
            );

            Ok::<Adapter, Box<dyn Error>>(adapter)
            /*//start_scan
            let discover = adapter.discover_devices().await?;
            // pin discover (not going out of scope in async)
            pin_mut!(discover);*/
            
        })?;

        let (tx1, rx1) = mpsc::channel::<u8>();
        let (tx2, rx2) = mpsc::channel::<u8>();
        let inner = InnerBluetooth::new(rx1, tx2);
        let t = rt.spawn( async {
            inner.discover(adapter).await
        });
        todo!()
    }   
}

/*
pub async fn new_bluetooth()->Result<, Box<dyn Error>>{
    //getting session
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;

    //reboot adapter
    adapter.set_powered(false).await?;
    adapter.set_powered(true).await?;

    println!(
        "Discovering on Bluetooth adapter {} with address {}\n",
        adapter.name(),
        adapter.address().await?
    );

    
    //start_scan
    let discover = adapter.discover_devices().await?;
    // pin discover (not going out of scope in async)
    pin_mut!(discover);
    
    todo!();
} 


impl InnerBluetooth{
    
}

impl Bluetooth{

    /// it create a new bluetooth device 
    pub fn new()->Self{
        let rt = Runtime::new().unwrap();
        let _guard = rt.enter();
        let t = block_on(async {
            new_bluetooth().await
        });
        
        todo!()
        
    }
}

impl Serial for Bluetooth{
    fn send(&mut self, d: u8) {
        todo!()
    }

    fn read(&mut self) -> u8 {
        todo!()
    }

    fn available(&mut self) -> i32 {
        todo!()
    }
}*/