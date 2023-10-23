#![feature(option_get_or_insert_default)]

mod assets;
mod camera;
mod map;
mod player;
mod utils;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_tnua::prelude::*;

use self::{camera::CameraPlugin, map::MapPlugin, player::PlayerPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default(),
            TnuaControllerPlugin,
            TnuaRapier3dPlugin,
        ))
        .add_plugins((CameraPlugin, PlayerPlugin, MapPlugin))
        .add_systems(PreStartup, assets::init)
        .add_systems(Startup, init)
        .run();
}

fn init(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.8 })),
            material: materials.add(Color::rgb(0.5, 0.3, 0.3).into()),
            transform: Transform::from_xyz(1., 1., 0.),
            ..Default::default()
        },
        Collider::cuboid(0.4, 0.4, 0.4),
        RigidBody::default(),
        Friction::coefficient(0.62),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.8 })),
            material: materials.add(Color::rgb(0.3, 0.3, 0.5).into()),
            transform: Transform::from_xyz(0., 1., 1.),
            ..Default::default()
        },
        Collider::cuboid(0.4, 0.4, 0.4),
        RigidBody::default(),
        Friction::coefficient(0.62),
    ));
}
