use bevy::app::App;
use bevy::prelude::Plugin;

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
        todo!()
    }
}
