use bevy::prelude::*;
use bevy_atmosphere::prelude::*;
use dolly::prelude::*;

#[derive(Default)]
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AtmospherePlugin)
            .add_systems(Startup, init)
            .add_systems(Update, (update_main_target, update_dolly_rigs));
    }
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct MainCameraTarget;

#[derive(Component)]
pub struct DollyRig {
    pub rig: CameraRig,
}

fn init(mut commands: Commands) {
    commands.spawn((
        MainCamera,
        Camera3dBundle {
            projection: PerspectiveProjection {
                fov: 25.0f32.to_radians(),
                ..default()
            }
            .into(),
            ..default()
        },
        AtmosphereCamera::default(),
        DollyRig {
            rig: CameraRig::builder()
                .with(Position::new(Vec3::ZERO))
                .with(YawPitch::new().pitch_degrees(-80.))
                .with(Smooth::new_position_rotation(0.8, 0.3))
                .with(Arm::new(Vec3::Z * 25.0))
                .build(),
        },
    ));
}

fn update_main_target(
    target: Query<&Transform, With<MainCameraTarget>>,
    mut camera: Query<&mut DollyRig, With<MainCamera>>,
    keys: Res<Input<KeyCode>>,
) {
    let target = target.single();
    let mut camera = camera.single_mut();

    camera.rig.driver_mut::<Position>().position = target.translation;

    if keys.pressed(KeyCode::E) {
        camera.rig.driver_mut::<YawPitch>().pitch_degrees = -20.;
        camera.rig.driver_mut::<Arm>().offset = Vec3::Z * 5.0;
    } else {
        camera.rig.driver_mut::<YawPitch>().pitch_degrees = -80.;
        camera.rig.driver_mut::<Arm>().offset = Vec3::Z * 25.0;
    };
}

fn update_dolly_rigs(mut query: Query<(&mut Transform, &mut DollyRig)>, time: Res<Time>) {
    for (mut transform, mut rig) in query.iter_mut() {
        rig.rig.update(time.delta_seconds());
        transform.translation = rig.rig.final_transform.position;
        transform.rotation = rig.rig.final_transform.rotation;
    }
}
