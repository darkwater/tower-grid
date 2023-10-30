mod assets;
mod camera;
mod hud;
mod map;
mod player;
mod utils;
mod world_tile;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_rapier3d::prelude::*;
use bevy_tnua::prelude::*;

use self::{camera::CameraPlugin, hud::HudPlugin, map::MapPlugin, player::PlayerPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            EguiPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            TnuaControllerPlugin,
            TnuaRapier3dPlugin,
        ))
        .add_plugins((CameraPlugin, HudPlugin, PlayerPlugin, MapPlugin))
        .add_systems(PreStartup, assets::init)
        .add_systems(Startup, init)
        .add_systems(Update, bevy::window::close_on_esc)
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
