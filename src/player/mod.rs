mod cursor;
mod input;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_tnua::prelude::*;
use leafwing_input_manager::prelude::*;

use self::input::Action;
use crate::{camera::MainCameraTarget, map::SpawnsChunksNearby};

#[derive(Default)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<Action>::default())
            .add_systems(Startup, (init, cursor::init))
            .add_systems(Update, (cursor::update_cursor,))
            .add_systems(Update, (input::move_player, input::hud_actions));
    }
}

#[derive(Component)]
pub struct Player;

impl Player {
    const HEIGHT: f32 = 1.8;
    const RADIUS: f32 = 0.3;

    const WALK_SPEED: f32 = 5.;
}

fn init(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    commands.spawn((
        Player,
        MainCameraTarget,
        SpawnsChunksNearby { radius_x: 2., radius_y: 1. },
        Collider::capsule_y((Player::HEIGHT - Player::RADIUS * 2.) / 2., Player::RADIUS),
        RigidBody::Dynamic,
        TnuaRapier3dIOBundle::default(),
        TnuaControllerBundle::default(),
        // TnuaRapier3dSensorShape(Collider::cylinder(0.1, radius * 0.9)),
        LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z,
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule {
                radius: Player::RADIUS,
                rings: 1,
                depth: Player::HEIGHT - Player::RADIUS * 2.,
                latitudes: 10,
                longitudes: 24,
                uv_profile: shape::CapsuleUvProfile::Aspect,
            })),
            material: materials.add(Color::rgb(0.5, 0.4, 0.3).into()),
            transform: Transform::from_xyz(0., Player::HEIGHT, 0.),
            ..Default::default()
        },
        input::manager_bundle(),
    ));
}
