use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    render::primitives::Aabb,
};

use bevy_aabb_instancing::{
    Cuboid, CuboidMaterial, CuboidMaterialMap, Cuboids, VertexPullingRenderPlugin, COLOR_MODE_RGB,
};

use rand::{thread_rng, Rng};

mod car;
mod infinite_grid;
//use protocol::::*;
use car::{Car, CarPlugin};
use infinite_grid::{InfiniteGrid, InfiniteGridPlugin};

#[derive(Component)]
struct MySimCamera;

fn main() {
    let mut app = App::new();
    app
        //defaulty plugins
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin {})
        .add_plugins(LogDiagnosticsPlugin::default())
        //aabb instancing plugin
        .add_plugins(VertexPullingRenderPlugin { outlines: true })
        //custom plugins
        .add_plugins(CarPlugin)
        .add_plugins(InfiniteGridPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, spawn_wall_2);
    app.run();
}

fn setup(
    mut commands: Commands,
    ass: Res<AssetServer>,
    mut material_map: ResMut<CuboidMaterialMap>,
) {
    let material_id = material_map.push(CuboidMaterial {
        color_mode: COLOR_MODE_RGB,
        ..default()
    });
    let instances = Vec::new();
    let cuboids = Cuboids::new(instances);
    let aabb = Aabb::from_min_max(
        Vec3::new(-1000.0, -1000., -1000.),
        Vec3::new(1000.0, 1000., 1000.),
    );
    commands
        .spawn(SpatialBundle::default())
        .insert((cuboids, aabb, material_id));

    // note that we have to include the `Scene0` label
    let my_gltf = ass.load("car.glb#Scene0");

    // to position our 3d model, simply use the Transform
    // in the SceneBundle
    commands
        .spawn((
            SceneBundle {
                scene: my_gltf,
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            },
            Car::default(),
            InfiniteGrid,
        ))
        .with_children(|x| {
            x.spawn((
                Camera3dBundle {
                    transform: Transform::from_xyz(0.0, 5.0, -20.0)
                        .looking_at(Vec3::new(0.0, 0.0, 2.0), Vec3::Y),
                    ..Default::default()
                },
                MySimCamera,
            ));
        });

    // directional 'sun' light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(-2.0, 2.0, -2.0),
            rotation: Quat::from_rotation_x(-1.),
            ..default()
        },
        ..default()
    });
}

fn spawn_wall_2(mut q: Query<&mut Cuboids>, keys: Res<Input<KeyCode>>) {
    if keys.pressed(KeyCode::I) {
        let mut t = q.iter_mut().next().unwrap();
        let mut rng = thread_rng();
        let mut v = Vec::new();
        let diff = Vec3::new(0.5, 0.5, 0.5);
        for _ in 0..100000 {
            let pos = Vec3::new(
                rng.gen_range(-1000.0..1000.),
                rng.gen_range(-1000.0..1000.),
                rng.gen_range(-1000.0..1000.),
            );
            
            let cuboid = Cuboid::new(pos - diff, pos + diff, (Color::WHITE * 0.5).as_rgba_u32());
            v.push(cuboid);
            //t.instances.push(cuboid);
        }
        t.instances.extend(v);

        println!("{}", t.instances.len());
    }
}
