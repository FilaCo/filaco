use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct SurroundedPlugin;

impl SurroundedPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

impl Plugin for SurroundedPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
    }
}

impl Default for SurroundedPlugin {
    fn default() -> Self {
        Self::new()
    }
}
