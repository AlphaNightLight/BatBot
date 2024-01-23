use std::{collections::VecDeque, error::Error};

use bluer::{
    gatt::{CharacteristicReader, CharacteristicWriter},
    AdapterEvent, Address, Device, Uuid,
};
use futures::{pin_mut, StreamExt};
use tokio::{io::AsyncWriteExt, runtime::Handle};

use super::Serial;

///Address::new([0x00, 0x18, 0x91, 0xD8, 0xE9, 0xC7])
pub struct Ble {
    to_send: Vec<u8>,
    handle: Handle,
    reader: CharacteristicReader,
    writer: CharacteristicWriter,
    read_buffer: VecDeque<u8>,
    address: Address,
    service_uuid: Uuid,
}

struct PreBle {
    reader: CharacteristicReader,
    writer: CharacteristicWriter,
}
impl Ble {
    /// Function to get a new bluetooth device
    ///
    /// it must be called with the desired bluetooth address, and an handle to a tokio runtime
    ///  
    pub fn try_new(
        address: Address,
        service_uuid: Uuid,
        handle: Handle,
    ) -> Result<Self, Box<dyn Error>> {
        let ble = handle.block_on(async { discover(address, service_uuid).await })?;
        let ret = Self {
            to_send: Vec::new(),
            handle,
            writer: ble.writer,
            reader: ble.reader,
            read_buffer: VecDeque::new(),
            address,
            service_uuid,
        };
        Ok(ret)
    }
    pub fn connect(&self) -> Result<Self, Box<dyn Error>> {
        let address = self.address;
        let service_uuid = self.service_uuid;
        let handle = self.handle.clone();
        Self::try_new(address, service_uuid, handle)
    }
}

/// trying to connect to the device 5 times
async fn connect(device: &Device) -> Result<(), Box<dyn Error>> {
    if !device.is_connected().await? {
        for _ in 0..4 {
            match device.connect().await {
                Ok(()) => return Ok(()),
                Err(err) => {
                    println!("    Connect error: {}", &err);
                }
            }
        }
        Err("impossible to connect")?
    } else {
        println!("    Already connected");
        Ok(())
    }
}
async fn get_characteristic(device: Device, service_uuid: Uuid) -> Result<PreBle, Box<dyn Error>> {
    println!("getting services");
    for service in device.services().await? {
        println!("{:?}", service.uuid().await);
        if service_uuid != service.uuid().await.unwrap() {
            continue;
        }
        println!("{:?}", service.uuid().await);
        let c = service.characteristics().await?.into_iter().next().unwrap();
        let r = c.notify_io().await?;
        let w = c.write_io().await?;
        return Ok(PreBle {
            reader: r,
            writer: w,
        });
    }
    todo!()
}

/// let's try to connect to this device
async fn discover(address: Address, service_uuid: Uuid) -> Result<PreBle, Box<dyn Error>> {
    println!("ble discovery");
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    adapter.set_powered(false).await?;
    //turn on adapter
    adapter.set_powered(true).await?;
    println!(
        "Discovering on Bluetooth adapter {} with address {}\n",
        adapter.name(),
        adapter.address().await?
    );

    // start scan
    //adapter.set_discovery_filter(bluer::DiscoveryFilter { uuids: HashSet::from_iter(vec![service_uuid]), transport: bluer::DiscoveryTransport::Le, ..Default::default() }).await?;
    let discover = adapter.discover_devices().await?;
    pin_mut!(discover);
    while let Some(evt) = discover.next().await {
        match evt {
            AdapterEvent::DeviceAdded(addr) => {
                if addr != address {
                    continue;
                }

                // if another device connected, let's try to find our characteristics
                let device = adapter.device(addr)?;
                println!("device {:?} {:?}", device, device.name().await);
                //starting connection
                connect(&device).await?;
                return get_characteristic(device.clone(), service_uuid).await;
            }
            AdapterEvent::DeviceRemoved(addr) => {
                println!("Removed device {addr}");
            }
            AdapterEvent::PropertyChanged(_) => {}
        }
    }
    Err("can't connect, the device haven't respond 10 times in a row")?
}

impl Serial for Ble {
    fn flush(&mut self) {
        //println!("{}", self.writer.mtu());
        let mtu = self.writer.mtu();
        self.to_send.chunks(mtu).for_each(|x| {
            self.handle.block_on(async {
                let _ = self.writer.write_all(x).await;
            });
        });
        self.to_send.clear();
        /*self.handle.block_on(async{
            if !self.device.is_connected().await.unwrap(){
                self.
                get_characteristic(device, service_uuid)
            }
        });*/
        /*self.handle.block_on(async {


            //

            self.writer.flush().await.unwrap();
            //self..flush().await.unwrap()
        });*/
    }

    fn send(&mut self, d: u8) {
        //println!("send");
        self.to_send.push(d);
    }

    fn read(&mut self) -> u8 {
        //println!("read");
        self.read_buffer.pop_front().unwrap_or(0)
    }

    fn available(&mut self) -> i32 {
        if let Ok(x) = self.reader.try_recv() {
            self.read_buffer.extend(x);
        }
        if let Ok(x) = self.reader.try_recv() {
            self.read_buffer.extend(x)
        }
        self.read_buffer.len() as i32
    }
}
