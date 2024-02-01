use bevy::{math::vec3, prelude::*};

use crate::input::Inputs;

#[derive(Component, Default)]
pub struct Car {
    pub speed: f32,
    pub rot_speed: f32,


    pub angle_cur:f32,
    pub obj: Vec3,
    pub angle_obj: f32,
}

fn move_car(mut car: Query<(&mut Transform, &mut Car)>, timer: Res<Time>, inputs: Res<Inputs>) {
    let correction=5.;
    let (mut transform, mut car) =  car.single_mut();
    let angle=car.angle_cur;
    let x_speed = (car.obj.x-transform.translation.x)*correction+ angle.sin()*car.speed*8.;
    let y_speed =0.;
    let z_speed = (car.obj.z-transform.translation.z)*correction+ angle.cos()*car.speed*8.;
    car.angle_cur += (inputs.rot_speed + (car.angle_obj-car.angle_cur)*correction)*timer.delta_seconds();
    transform.translation+=vec3(x_speed, y_speed, z_speed)*timer.delta_seconds();
    transform.rotation=Quat::IDENTITY;
    transform.rotate_y(car.angle_cur);



    /*
    car.rot_speed=inputs.rot_speed;
    car.speed=inputs.speed;
    let forward = transform.local_z();
    transform.rotate_local_y(car.rot_speed * timer.delta_seconds());
    transform.translation += forward * car.speed * timer.delta_seconds() * 8.;*/
}
/*
/// process input from joystick and similar and
fn input_car_update(
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
    mut cars: Query<&mut Car>,
    keys: Res<Input<KeyCode>>,
) {
    let mut car = cars.iter_mut().next().unwrap();
    car.speed = 0.;
    car.rot_speed = 0.;
    for g in gamepads.iter() {
        let y = axes.get(GamepadAxis::new(g, GamepadAxisType::LeftStickY));
        let x = axes.get(GamepadAxis::new(g, GamepadAxisType::LeftStickX));
        if x.is_none() || y.is_none() {
            continue;
        }
        let y = y.unwrap();
        let x = x.unwrap();

        car.speed = y;
        car.rot_speed = -x;
    }
    if keys.pressed(KeyCode::W) {
        car.speed += 1.;
    }
    if keys.pressed(KeyCode::S) {
        car.speed -= 1.;
    }
    if keys.pressed(KeyCode::A) {
        car.rot_speed += 1.;
    }
    if keys.pressed(KeyCode::D) {
        car.rot_speed -= 1.;
    }
}*/
pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_car); //(input_car_update, move_car)
    }
}
