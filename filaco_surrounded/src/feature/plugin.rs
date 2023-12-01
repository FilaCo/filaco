use crate::feature::player::PlayerPlugin;
use bevy::app::{App, Plugin};
use bevy::DefaultPlugins;

pub struct SurroundedPlugin;

impl SurroundedPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

impl Plugin for SurroundedPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((DefaultPlugins, PlayerPlugin));
    }
}

impl Default for SurroundedPlugin {
    fn default() -> Self {
        Self::new()
    }
}
