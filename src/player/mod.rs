use bevy::prelude::*;

use crate::map::SpawnsChunksNearby;

#[derive(Default)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init)
            .add_systems(Update, move_player);
    }
}

#[derive(Component)]
struct Player {}

fn init(mut commands: Commands) {
    commands.spawn((
        Player {},
        Transform::default(),
        SpawnsChunksNearby {
            radius_x: 2.,
            radius_y: 1.,
        },
    ));
}

fn move_player(mut player: Query<&mut Transform, With<Player>>, time: Res<Time>) {
    player.single_mut().translation.z += time.delta_seconds() * 10.;
}
