use bevy::app::App;
use bevy::prelude::{
    default, Camera3dBundle, Commands, Component, Plugin, Startup, Transform, Vec3,
};

pub struct CameraPlugin;

impl CameraPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for CameraPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}

#[derive(Component)]
struct InGameCamera;

#[derive(Component)]
struct MenuCamera;

fn setup_camera(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(-3.0, 3.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(InGameCamera);
}
