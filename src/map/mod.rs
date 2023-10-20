use bevy::{
    prelude::{shape::Plane, *},
    utils::HashSet,
};

use crate::assets::AssetHandles;

pub const CHUNK_SIZE: f32 = 16.;

#[derive(Default)]
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init)
            .add_systems(Update, (spawn_new_chunks, populate_new_chunks));
    }
}

fn init(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::PI / 4.)),
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 10000.,
            ..default()
        },
        ..default()
    });
}

/// Will cause new chunks to spawn nearby, within an AABB denoted by the radii.
///
/// eg. `radius_x` = 2, `radius_y` = 1 will spawn chunks in a 5x3 area around the entity.
/// Note that these values are in chunks, so multiply by `CHNUK_SIZE` to get the area in meters.
#[derive(Component)]
pub struct SpawnsChunksNearby {
    pub radius_x: f32,
    pub radius_y: f32,
}

#[derive(Component)]
pub struct Chunk {
    pub x: i32,
    pub z: i32,
}

fn spawn_new_chunks(
    chunks: Query<&Chunk>,
    triggers: Query<(&Transform, &SpawnsChunksNearby), Changed<Transform>>,
    mut commands: Commands,
) {
    let mut queue = HashSet::new();

    for (transform, scn) in triggers.iter() {
        let pos = transform.translation;
        let x = pos.x / CHUNK_SIZE;
        let z = pos.z / CHUNK_SIZE;

        for x in (x - scn.radius_x).floor() as i32..(x + scn.radius_x).ceil() as i32 {
            for z in (z - scn.radius_y).floor() as i32..(z + scn.radius_y).ceil() as i32 {
                queue.insert((x, z));
            }
        }
    }

    for (x, z) in queue.drain() {
        if chunks.iter().any(|chunk| chunk.x == x && chunk.z == z) {
            continue;
        }

        commands.spawn(Chunk { x, z });
    }
}

fn populate_new_chunks(
    chunks: Query<(&Chunk, Entity), Without<Transform>>,
    assets: Res<AssetHandles>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
) {
    for (chunk, entity) in chunks.iter() {
        commands.entity(entity).insert(PbrBundle {
            transform: Transform::from_translation(Vec3::new(
                chunk.x as f32 * CHUNK_SIZE,
                0.,
                chunk.z as f32 * CHUNK_SIZE,
            )),
            mesh: meshes.add(
                Plane {
                    size: CHUNK_SIZE,
                    subdivisions: 0,
                }
                .into(),
            ),
            material: assets.ground_material.clone(),
            ..default()
        });
    }
}
