mod assets;
mod camera;
mod map;
mod player;
mod utils;

use bevy::prelude::*;

use self::{camera::CameraPlugin, map::MapPlugin, player::PlayerPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((CameraPlugin, PlayerPlugin, MapPlugin))
        .add_systems(PreStartup, assets::init)
        .run();
}
