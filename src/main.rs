#![feature(str_as_str)]

use bevy::prelude::*;

use std::f32::consts::PI;

mod brick_color;
mod instance;
mod xml_parse;
use instance::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, InstancePlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-500.0, 500.0, 500.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
