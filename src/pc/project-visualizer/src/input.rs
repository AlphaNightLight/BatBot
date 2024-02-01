use bevy::{app::{Plugin, Update}, ecs::system::{Res, ResMut, Resource}, input::{gamepad::{GamepadAxis, GamepadAxisType, Gamepads}, keyboard::KeyCode, Axis, Input}};



#[derive(Resource)]
pub struct Inputs{
    pub speed: f32,
    pub rot_speed: f32,
}


fn input_update(
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
    mut inputs: ResMut<Inputs>,
    keys: Res<Input<KeyCode>>,
) {
    inputs.speed = 0.;
    inputs.rot_speed = 0.;
    for g in gamepads.iter() {
        let y = axes.get(GamepadAxis::new(g, GamepadAxisType::LeftStickY));
        let x = axes.get(GamepadAxis::new(g, GamepadAxisType::LeftStickX));
        if x.is_none() || y.is_none() {
            continue;
        }
        let y = y.unwrap();
        let x = x.unwrap();

        inputs.speed = y;
        inputs.rot_speed = -x;
    }
    if keys.pressed(KeyCode::W) {
        inputs.speed += 1.;
    }
    if keys.pressed(KeyCode::S) {
        inputs.speed -= 1.;
    }
    if keys.pressed(KeyCode::A) {
        inputs.rot_speed += 1.;
    }
    if keys.pressed(KeyCode::D) {
        inputs.rot_speed -= 1.;
    }
}


pub struct InputsPlugin;
impl Plugin for InputsPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(Inputs{speed: 0., rot_speed: 0.}).add_systems(Update, input_update);
    }
}