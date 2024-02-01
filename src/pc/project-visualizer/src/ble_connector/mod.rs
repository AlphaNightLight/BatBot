mod data;
use std::error::Error;
use std::time::Instant;

use bevy::prelude::*;
use bevy_aabb_instancing::Cuboids;
use protocol::serial::tokio::runtime::Handle;
use protocol::serial::{tokio::runtime::Runtime, Address, Uuid};
use protocol::{serial::Ble, Protocol};

use crate::ble_connector::data::Joystick;
use crate::car::Car;
use crate::input::Inputs;
use crate::spawn_wall;

use self::data::{CarPosition, Position};

//#[derive(Resource)]

fn connect(handle: Handle) -> Result<Protocol<Ble>, Box<dyn Error>> {
    let bl = Ble::try_new(
        Address::new([0xA8, 0x10, 0x87, 0x67, 0x73, 0x2A]),
        Uuid::from_bytes([
            0x00, 0x00, 0xFF, 0xE0, 0x00, 0x00, 0x10, 0x00, 0x80, 0x00, 0x00, 0x80, 0x5F, 0x9B,
            0x34, 0xFB,
        ]),
        handle,
    )?;
    Ok(Protocol::new(bl))
}

impl Default for BleResource {
    fn default() -> Self {
        let runtime = Runtime::new().unwrap();
        let protocol = connect(runtime.handle().clone()).unwrap();
        Self {
            runtime,
            protocol,
            last_joystick: Instant::now(),
            unavailable_wave: 0,
        }
    }
}

pub struct BleResource {
    runtime: Runtime,
    protocol: Protocol<Ble>,
    unavailable_wave: usize,
    last_joystick: Instant,
}

pub fn setup(world: &mut World) {
    world.insert_non_send_resource(BleResource::default())
}

pub fn receive(mut res: NonSendMut<BleResource>, mut q: Query<&mut Cuboids>, mut car: Query<(&mut Transform, &mut Car)>) {
    for _ in 0..10 {
        if let Some(data) = res.protocol.read() {
            //println!("{:?}", String::from_utf8(data.clone()));
            res.unavailable_wave = 0;
            if let Ok(pos) = Position::try_from(&data[..]) {
                let (t, _) = car.get_single().unwrap();
                let cuboids = q.iter_mut().next().unwrap();
                spawn_wall(cuboids, t.translation.x+pos.x, t.translation.y+pos.z, t.translation.z+pos.y);
                //println!("{pos:?}");
            }
            if let Ok(pos) = CarPosition::try_from(&data[..]) {
                let (mut transform, mut car) = car.get_single_mut().unwrap();
                car.obj=Vec3{x: pos.x, y: pos.z, z: pos.y};
                car.angle_obj= pos.angle;
                /*transform.translation.x=pos.x;
                transform.translation.y=pos.z;
                transform.translation.z=pos.y;
                transform.rotation=Quat::IDENTITY;
                transform.rotate_y(pos.angle);*/

                //println!("x= {}, y={}", pos.x, pos.y);
            }
        } else {
            res.unavailable_wave += 1;
            if res.unavailable_wave <= 500 {
                continue;
            }

            if let Ok(protocol) = connect(res.runtime.handle().clone()) {
                res.protocol = protocol;
            }
        }
    }
}
pub fn send(mut res: NonSendMut<BleResource>, inputs: Res<Inputs>){
    if res.last_joystick.elapsed().as_millis()>20{
        res.last_joystick=Instant::now();
        //println!("sending");
        let j = Joystick{x: inputs.speed, y: inputs.rot_speed};
        res.protocol.send_struct(j);
    }
}

pub struct BlePlugin;

impl Plugin for BlePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, receive)
            .add_systems(Update, send);
    }
}
