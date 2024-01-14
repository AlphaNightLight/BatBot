use std::f32::consts::PI;

use bevy::prelude::*;

#[derive(Default, Component)]
pub struct InfiniteGrid;

fn draw_origin(gizmos: &mut Gizmos) {
    gizmos.sphere(Vec3::new(0., 0.0, 0.), Quat::IDENTITY, 0.5, Color::RED);
}

fn draw_grid(x: f32, z: f32, gizmos: &mut Gizmos) {
    let x = x.floor();
    let z = z.floor();
    for xi in -10..=10 {
        for zi in -10..=10 {
            gizmos.rect(
                Vec3::new(x + xi as f32 + 0.5, 0.0, z + zi as f32 + 0.5),
                Quat::from_rotation_x(PI / 2.),
                Vec2::new(1.0, 1.0),
                Color::WHITE * 0.5,
            )
        }
    }
}

fn draw_infinite_grid(q: Query<(&InfiniteGrid, &Transform)>, mut gizmos: Gizmos) {
    //draw origin
    draw_origin(&mut gizmos);
    for (_, t) in q.iter() {
        draw_grid(t.translation.x, t.translation.z, &mut gizmos)
    }
}

pub struct InfiniteGridPlugin;

impl Plugin for InfiniteGridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_infinite_grid);
    }
}
