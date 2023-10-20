use bevy::prelude::*;
use bevy_atmosphere::prelude::*;
use bevy_dolly::prelude::*;

#[derive(Default)]
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AtmospherePlugin)
            .add_systems(Startup, init)
            .add_systems(Update, Dolly::<MainCamera>::update_active)
            .add_systems(Update, update);
    }
}

#[derive(Component)]
struct MainCamera;

fn init(mut commands: Commands) {
    commands.spawn((
        MainCamera,
        Rig::builder()
            .with(Position::new(Vec3::ZERO))
            .with(YawPitch::new().pitch_degrees(-10.))
            .with(Smooth::new_position(0.3))
            .with(Smooth::new_rotation(0.3))
            .with(Arm::new(Vec3::Z * 14.0))
            .build(),
        Camera3dBundle::default(),
        AtmosphereCamera::default(),
    ));
}

fn update(
    mut query: Query<(&mut Transform, &mut Rig), With<MainCamera>>,
    // player: Query<&Transform, With<Player>>,
) {
}
