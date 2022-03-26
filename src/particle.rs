use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use nanorand::{Rng, WyRand};

use crate::{
    motion::{Angle, Position, Velocity},
    utils::cursor_world_position,
};

#[derive(Component)]
pub struct Particle {
    size: f32,
    hue: f32,
}

impl Particle {
    pub fn spawn(
        commands: &mut Commands,
        size: f32,
        position: Position,
        velocity: Velocity,
        angle: Angle,
        hue: f32,
    ) {
        let shape = shapes::Circle {
            radius: size / 2.0,
            center: Vec2::ZERO,
        };

        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &shape,
                DrawMode::Fill(FillMode::color(Color::Hsla {
                    hue,
                    saturation: 1.0,
                    lightness: 0.5,
                    alpha: 0.5,
                })),
                Transform::from_xyz(position.0.x, position.0.y, 1.0),
            ))
            .insert(Particle { size, hue })
            .insert(position)
            .insert(velocity)
            .insert(angle);
    }
}

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(input_handler).add_system(shrink);
    }
}

fn input_handler(
    mut commands: Commands,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut rng: ResMut<WyRand>,
    time: Res<Time>,
) {
    let hue = ((time.seconds_since_startup() * 100.0) % 360.0) as f32;

    let (camera, camera_transform) = q_camera.single();

    let window = windows.get(camera.window).unwrap();

    let cusor_world_pos = cursor_world_position(window, camera, camera_transform);

    if buttons.just_pressed(MouseButton::Left) {
        if let Some(cursor_pos) = cusor_world_pos {
            (0..50).for_each(|_| {
                Particle::spawn(
                    &mut commands,
                    rng.generate_range(10..=50) as f32,
                    cursor_pos,
                    Velocity(rng.generate_range(150..=250) as f32),
                    Angle::from_grad(rng.generate_range(0..360) as f32),
                    hue,
                );
            });
        }
    }
    if buttons.pressed(MouseButton::Left) {
        if let Some(cursor_pos) = cusor_world_pos {
            (0..5).for_each(|i| {
                let part = 1 + i % 4;
                let part = 3 * hue as i32 + part * 90;

                Particle::spawn(
                    &mut commands,
                    rng.generate_range(10..=40) as f32,
                    cursor_pos,
                    Velocity(rng.generate_range(150..=250) as f32),
                    Angle::from_grad(rng.generate_range(part - 5..part + 5) as f32),
                    hue,
                );
            });
        }
    }
}

fn shrink(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Path, &mut Particle)>,
    time: Res<Time>,
) {
    for (entity, mut path, mut particle) in query.iter_mut() {
        if particle.size > 0.2 {
            particle.size -= 15.0 * time.delta_seconds();

            let shape = shapes::Circle {
                radius: particle.size / 2.0,
                center: Vec2::ZERO,
            };
            *path = ShapePath::build_as(&shape);
        } else {
            commands.entity(entity).despawn_recursive();
        }
    }
}
