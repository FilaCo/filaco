use bevy::app::App;
use bevy::prelude::{
    default, shape, Assets, Color, Commands, Mesh, PbrBundle, Plugin, PointLight, PointLightBundle,
    ResMut, StandardMaterial, Startup, Transform,
};
use bevy_rapier3d::prelude::{Collider, RigidBody};

pub struct MapPlugin;

impl MapPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for MapPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_map);
    }
}

fn setup_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(50.).into()),
            material: materials.add(Color::SILVER.into()),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(50., 5., 50.));

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8., 16., 8.),
        ..default()
    });
}
