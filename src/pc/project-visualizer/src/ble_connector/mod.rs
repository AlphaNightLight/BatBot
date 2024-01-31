
mod data;
use std::error::Error;
use std::sync::mpsc::Receiver;
use std::sync::Mutex;
use std::time::Instant;

use bevy::ecs::storage::ResourceData;
use bevy::prelude::*;
use protocol::serial::tokio::runtime::Handle;
use protocol::{serial::Ble, Protocol};
use protocol::serial::{tokio::runtime::Runtime, Address, Uuid};

use data::Joystick;

use self::data::Position;

//#[derive(Resource)]


fn connect(handle: Handle)->Result<Protocol<Ble>, Box<dyn Error>>{
    let bl = Ble::try_new(
        Address::new([0xA8, 0x10, 0x87, 0x67, 0x73, 0x2A]),
        Uuid::from_bytes([
            0x00, 0x00, 0xFF, 0xE0, 0x00, 0x00, 0x10, 0x00, 0x80, 0x00, 0x00, 0x80, 0x5F, 0x9B, 0x34, 0xFB,
        ]),
        handle,
    )?;
    Ok(Protocol::new(bl))
}

impl Default for BleResource{
    fn default() -> Self {
        let runtime = Runtime::new().unwrap();
        let protocol= connect(runtime.handle().clone()).unwrap();
        Self { runtime, protocol, last_joystick: Instant::now(), last_read: Instant::now(), unavailable_wave: 0}
    }
}

pub struct BleResource{
    runtime: Runtime,
    protocol: Protocol<Ble>,
    unavailable_wave: usize,
    last_joystick: Instant,
    last_read: Instant,
}

pub fn setup(world: &mut World){
    /*let runtime = Runtime::new().unwrap();
    let bl = Ble::try_new(
        Address::new([0xA8, 0x10, 0x87, 0x67, 0x73, 0x2A]),
        Uuid::from_bytes([
            0x00, 0x00, 0xFF, 0xE0, 0x00, 0x00, 0x10, 0x00, 0x80, 0x00, 0x00, 0x80, 0x5F, 0x9B, 0x34, 0xFB,
        ]),
        runtime.handle().clone(),
    )
    .unwrap();
    let mut protocol = Protocol::new(bl);
    let t = Joystick{x: 0., y: 0.};
    protocol.send_struct(t);*/
    world.insert_non_send_resource(BleResource::default())
}

pub fn receive(mut res: NonSendMut<BleResource>){
    let handle= res.runtime.handle().clone();
    handle.block_on(async{
        for _ in 0..10{
            if let Some(data)= res.protocol.read(){
                println!("{:?}", String::from_utf8(data.clone()));
                res.unavailable_wave=0;
                if let Ok(x) = Position::try_from(&data[..]){
                    println!("{x:?}");
                }
            }else{
                res.unavailable_wave+=1;
                if res.unavailable_wave<=500{
                    continue;
                }

                if let Ok(protocol) = connect(res.runtime.handle().clone()){
                    res.protocol=protocol;
                }
            }
        }
        

    })
    
}



pub struct BlePlugin;

impl Plugin for BlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, receive);
        //app.insert_resource(BleResource::default());
    }
}