use bevy::prelude::*;

#[derive(Resource)]
pub struct AssetHandles {
    pub ground_material: Handle<StandardMaterial>,
    pub concrete_material: Handle<StandardMaterial>,
}

pub fn init(mut materials: ResMut<Assets<StandardMaterial>>, mut commands: Commands) {
    commands.insert_resource(AssetHandles {
        ground_material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.881136, 0.789159, 0.655327),
            perceptual_roughness: 0.8,
            ..default()
        }),
        concrete_material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.3365, 0.3349, 0.3285),
            perceptual_roughness: 0.6,
            ..default()
        }),
    });
}
