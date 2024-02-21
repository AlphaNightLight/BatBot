mod data;
use std::error::Error;
use std::time::{Duration, Instant};

use bevy::ecs::system::InsertResource;
use bevy::prelude::*;
use bevy_aabb_instancing::Cuboids;
use protocol::serial::tokio::runtime::Handle;
use protocol::serial::{tokio::runtime::Runtime, Address, Uuid};
use protocol::{serial::Ble, Protocol};

use crate::ble_connector::data::Joystick;
use crate::car::Car;
use crate::input::Inputs;
use crate::spawn_wall;

use self::data::{BlockRow, CarPosition, Position};

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
            blocks_instant: Instant::now(),
            positions_instant: Instant::now(),
        }
    }
}

pub struct BleResource {
    runtime: Runtime,
    protocol: Protocol<Ble>,
    unavailable_wave: usize,
    last_joystick: Instant,
    blocks_instant: Instant,
    positions_instant: Instant,
}
#[derive(Resource, Default)]
pub struct BleStatistics{
    pub n_blocks: usize,
    pub n_pos: usize,
    pub dur_blocks: Duration,
    pub dur_pos: Duration,
}

pub fn setup(world: &mut World) {
    world.insert_non_send_resource(BleResource::default())
}

pub fn receive(mut res: NonSendMut<BleResource>, mut q: Query<&mut Cuboids>, mut car: Query<(&mut Transform, &mut Car)>, mut ble_stats: ResMut<BleStatistics>) {
    for _ in 0..10 {
        if let Some(data) = res.protocol.read() {
           
            println!("{:?}", String::from_utf8(data.clone()));
            res.unavailable_wave = 0;
            if let Ok(row) = BlockRow::try_from(&data[..]) {
                println!("{:?}", row);
                ble_stats.n_blocks+=8;
                ble_stats.dur_blocks+=res.blocks_instant.elapsed();

                res.blocks_instant=Instant::now();
                let (t, car) = car.get_single().unwrap();
                

                let y = row.index;
                for x in 0..8{
                    let dist = row.row[x];
                    let a_orizzontale = (-1.+(x as f32-4.)*2.)*30./7.;
                    let a_verticale = (-1.+(y as f32-4.)*2.)*30./7.;
                    let y = dist * a_orizzontale.sin();
                    let x = dist*(a_verticale+car.angle_cur).cos();
                    let z = dist*(a_verticale+car.angle_cur).sin();
                    let cuboids = q.iter_mut().next().unwrap();
                    spawn_wall(cuboids, 
                        t.translation.x+x, 
                        t.translation.y+z, 
                        t.translation.z+y);
                }
                
                //println!("{pos: ?}");  
            }
            if let Ok(pos) = CarPosition::try_from(&data[..]) {
                ble_stats.n_pos+=1;
                ble_stats.dur_pos+=res.positions_instant.elapsed();
                res.positions_instant=Instant::now();
                let (_, mut car) = car.get_single_mut().unwrap();
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
            .add_systems(Update, send)
            .insert_resource(BleStatistics::default());
    }
}
