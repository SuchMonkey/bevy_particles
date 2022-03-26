use std::f32::consts::PI;

use bevy::prelude::*;

#[derive(Component, Copy, Clone, Debug, PartialEq)]
pub struct Position(pub Vec2);

#[derive(Component, Copy, Clone, Debug, PartialEq)]
pub struct Velocity(pub f32);

#[derive(Component, Copy, Clone, Debug, PartialEq)]
pub struct Angle(pub f32);

impl Angle {
    pub fn from_grad(grad: f32) -> Self {
        Angle(grad * 2.0 * PI / 360.0)
    }
}

pub struct MotionPlugin;

impl Plugin for MotionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_position);
    }
}

fn update_position(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Position, &Velocity, &Angle)>,
) {
    let delta = time.delta_seconds();
    for (mut transform, mut position, velocity, direction) in query.iter_mut() {
        position.0.x += velocity.0 * delta * direction.0.cos();
        position.0.y += velocity.0 * delta * direction.0.sin();

        transform.translation = Vec3::new(position.0.x, position.0.y, 1.0);
    }
}
