use bevy::app::{App, Plugin};
use bevy::prelude::{Bundle, Component};

pub struct PlayerPlugin;

impl PlayerPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for PlayerPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Bundle)]
pub struct PlayerBundle {
    marker: Player,
}
