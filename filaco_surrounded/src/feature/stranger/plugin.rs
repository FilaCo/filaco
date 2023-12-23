use crate::feature::stranger::camera::CameraPlugin;
use crate::feature::stranger::input::InputPlugin;
use bevy::prelude::{App, Component, Plugin};

pub struct StrangerPlugin;

impl StrangerPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for StrangerPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for StrangerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CameraPlugin).add_plugins(InputPlugin);
    }
}

#[derive(Component)]
struct Stranger;
