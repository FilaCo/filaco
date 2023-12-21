use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::collections::HashMap;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .run()
}

fn handle_input(scan_codes: Res<Input<ScanCode>>) {}

#[derive(Component)]
struct Stranger;
git
#[derive(Component)]
struct InGameCamera;

#[derive(Component)]
struct MenuCamera;

struct GameState {
    stage: PlotStage,
}

enum PlotStage {
    Childhood,
    Adolescence,
    Youth,
    Final,
}
