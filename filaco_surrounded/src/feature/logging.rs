use bevy::app::App;
use bevy::prelude::Plugin;

pub struct LogPlugin;

impl LogPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for LogPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for LogPlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}
