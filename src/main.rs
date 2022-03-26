use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use nanorand::WyRand;

use crate::{motion::MotionPlugin, particle::ParticlePlugin};

mod motion;
mod particle;

mod utils;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WyRand::new())
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(MotionPlugin)
        .add_plugin(ParticlePlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
