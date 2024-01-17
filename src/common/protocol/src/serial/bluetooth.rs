use std::{error::Error, sync::mpsc::{self, Sender, Receiver}};

use bluer::{AdapterEvent, Adapter, Address};
use futures::{pin_mut, StreamExt, Stream, executor::block_on, TryFutureExt};
use tokio::{runtime::Runtime, task::spawn_blocking};
use std::future::Future;

use super::Serial;


struct InnerBluetooth{
    to_send: mpsc::Receiver<u8>,
    readen: mpsc::Sender<u8>,
    address: Address,
}

impl InnerBluetooth{
    fn new(to_send: Receiver<u8>, readen: Sender<u8>, address: Address)->Self{
        Self {
            to_send,
            readen,
            address
        }
    }
    pub async fn discover(self, adapter: Adapter)->Result<(), Box<dyn Error>>{
        let discover = adapter.discover_devices().await?;
        // pin discover (not going out of scope in async)
        pin_mut!(discover);
        while let Some(evt) = discover.next().await{
            match evt{
                AdapterEvent::DeviceAdded(x) => {
                    if self.address != x{
                        continue;
                    }
                    println!("found, trying connecting:");
                    let device = adapter.device(x)?;
                    let mut count = 0;
                    while let Err(x) =  device.connect().await{
                        count+=1;
                        if count>10{
                            Err("Can't connect".to_string())?;
                        }
                        println!("{}", x);
                    }

                },
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
    pub fn new(address: Address)->Result<Self, Box<dyn Error>>{
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
        let inner = InnerBluetooth::new(rx1, tx2, address);
        async fn test(inner: InnerBluetooth, adapter: Adapter)->Result<(), String>{
            inner.discover(adapter).map_err(|x| x.to_string()).await?;

            Ok(())
        }
        /*let closure = (async{
            //let inner=inner;
            //inner.discover(adapter).await.map_err(|x| x.to_string());
            Ok::<(), ()>(())
        })();*/
        let t = rt.spawn(test(inner, adapter));
        //inner.discover(adapter); 
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