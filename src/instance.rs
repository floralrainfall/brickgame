use bevy::prelude::*;
use std::option::Option;
use std::vec::Vec;

use crate::brick_color::*;
use crate::xml_parse::*;

pub struct InstancePlugin;
impl Plugin for InstancePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_instances);
        app.add_systems(Update, update_instances);
        app.add_systems(Update, update_parts_dirty);
    }
}

#[derive(Component)]
pub struct Instance {
    pub class: String,
    pub parent: Option<Entity>,
    pub children: Vec<Entity>,
}

#[derive(Component)]
pub struct Part {
    pub size: Vec3,
    pub dirty: bool,
    pub needsMesh: bool,
    pub brickColor: i32,
}

impl Part {
    pub fn new() -> Self {
        Self {
            size: Vec3::ONE,
            dirty: true,
            brickColor: 0,
            needsMesh: true,
        }
    }
}

#[derive(Bundle)]
pub struct PartBundle {
    pub mesh: Mesh3d,
    pub material: MeshMaterial3d<StandardMaterial>,
}

impl PartBundle {
    pub fn new(
        mut materials: &mut ResMut<Assets<StandardMaterial>>,
        mut meshes: &mut ResMut<Assets<Mesh>>,
        color: Srgba,
    ) -> Self {
        Self {
            mesh: Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            material: MeshMaterial3d(materials.add(StandardMaterial {
                base_color: color.into(),
                ..Default::default()
            })),
        }
    }
}

impl Instance {
    pub fn new(class: &str, parent: Option<Entity>) -> Self {
        Self {
            class: String::from(class),
            parent: parent,
            children: Vec::new(),
        }
    }
}

fn update_instances() {}

fn update_parts_dirty(
    mut query: Query<(&mut Part, &mut Transform, Entity), With<Part>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
) {
    let mut meshes_generated = 0;
    for (mut part, mut transform, entity) in &mut query {
        if part.needsMesh {
            commands.entity(entity).insert(PartBundle::new(
                &mut materials,
                &mut meshes,
                brick_color_to_srgb(part.brickColor),
            ));
            part.needsMesh = false;
            part.dirty = false;
            transform.scale = part.size;
            meshes_generated += 1;
        }
    }
    if meshes_generated != 0 {
        info!("{} meshes generated at system", meshes_generated);
    }
}
