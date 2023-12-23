use bevy::app::App;
use bevy::prelude::Plugin;

pub struct InputPlugin;

impl InputPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for InputPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {}
}
