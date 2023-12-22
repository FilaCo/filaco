use bevy::prelude::{App, Commands, Component, Entity, Event, Plugin, Query, Update, Vec3, With};
use bevy_rapier3d::prelude::Velocity;

pub struct MovementPlugin;

impl MovementPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for MovementPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_movement);
    }
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub enum Move {
    Forward,
    Backward,
    Left,
    Right,
}

fn handle_movement(mut commands: Commands, mut q: Query<(Entity, &Move, Option<&mut Velocity>)>) {
    for (entity, move_direction, mut velocity) in q.iter_mut() {
        match move_direction {
            Move::Forward => {}
            Move::Backward => {}
            Move::Left => {}
            Move::Right => {}
        };

        commands.entity(entity).remove::<Move>();
    }
}
