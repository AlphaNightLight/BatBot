use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Car {
    speed: f32,
    rot_speed: f32,
}

fn move_car(mut car: Query<(&mut Transform, &mut Car)>, timer: Res<Time>) {
    for (mut transform, car) in &mut car {
        let forward = transform.local_z();
        transform.rotate_local_y(car.rot_speed * timer.delta_seconds());
        transform.translation += forward * car.speed * timer.delta_seconds() * 8.;
    }
}

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
}
pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (input_car_update, move_car));
    }
}
