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
            .add_systems(Startup, init)
            .add_systems(Update, (input::move_player,));
    }
}

#[derive(Component)]
pub struct Player {
    pub walk_speed: f32,
}

fn init(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    let length = 1.8;
    let radius = 0.3;

    commands.spawn((
        Player { walk_speed: 5. },
        MainCameraTarget,
        SpawnsChunksNearby { radius_x: 2., radius_y: 1. },
        // KinematicCharacterController {
        //     up: Vec3::Y,
        //     slide: true,
        //     offset: CharacterLength::Relative(0.05),
        //     ..default()
        // },
        Collider::capsule_y((length - radius * 2.) / 2., radius),
        RigidBody::Dynamic,
        TnuaRapier3dIOBundle::default(),
        TnuaControllerBundle::default(),
        LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z,
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule {
                radius,
                rings: 1,
                depth: length - radius * 2.,
                latitudes: 10,
                longitudes: 24,
                uv_profile: shape::CapsuleUvProfile::Aspect,
            })),
            material: materials.add(Color::rgb(0.5, 0.4, 0.3).into()),
            transform: Transform::from_xyz(0., length, 0.),
            ..Default::default()
        },
        input::manager_bundle(),
    ));
}

// pub fn apply_gravity(
//     mut player: Query<&mut KinematicCharacterController, With<Player>>,
//     time: Res<Time>,
//     rapier: Res<RapierConfiguration>,
// ) {
//     let mut kcc = player.single_mut();

//     *kcc.translation.get_or_insert_default() += rapier.gravity * time.delta_seconds();
// }
