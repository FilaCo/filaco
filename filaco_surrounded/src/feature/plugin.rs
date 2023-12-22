use crate::feature::logging::LogPlugin;
use crate::feature::map::MapPlugin;
use crate::feature::movement::MovementPlugin;
use bevy::prelude::{App, Plugin};
use bevy::DefaultPlugins;
use bevy_rapier3d::prelude::{NoUserData, RapierPhysicsPlugin};

pub struct SurroundedPlugin;

impl SurroundedPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for SurroundedPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for SurroundedPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugins(LogPlugin)
            .add_plugins(MapPlugin)
            .add_plugins(MovementPlugin);
    }
}
