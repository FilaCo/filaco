use crate::feature::stranger::camera::{CameraPlugin, InGameCamera};
use bevy::prelude::{
    default, info, App, Commands, Component, Plugin, Quat, Query, Reflect, Res, Startup, Transform,
    Update, Window, With,
};
use bevy_rapier3d::prelude::{Collider, KinematicCharacterController, RigidBody};
use leafwing_input_manager::prelude::{
    ActionState, DualAxis, InputManagerPlugin, InputMap, QwertyScanCode,
};
use leafwing_input_manager::{Actionlike, InputManagerBundle};

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
        app.add_plugins(InputManagerPlugin::<StrangerAction>::default())
            .add_plugins(CameraPlugin)
            .add_systems(Startup, setup_stranger)
            .add_systems(Update, handle_stranger_actions);
    }
}

#[derive(Component)]
struct Stranger;

#[derive(Actionlike, Clone, Debug, Copy, PartialEq, Eq, Hash, Reflect)]
enum StrangerAction {
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
    LookAround,
}

fn setup_stranger(mut commands: Commands) {
    commands
        .spawn(Stranger)
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::capsule_y(0.5, 0.5))
        .insert(KinematicCharacterController::default())
        .insert(InputManagerBundle::<StrangerAction> {
            input_map: InputMap::default()
                .insert(QwertyScanCode::W, StrangerAction::MoveForward)
                .insert(QwertyScanCode::S, StrangerAction::MoveBackward)
                .insert(QwertyScanCode::A, StrangerAction::MoveLeft)
                .insert(QwertyScanCode::D, StrangerAction::MoveRight)
                .insert(DualAxis::mouse_motion(), StrangerAction::LookAround)
                .build(),
            ..default()
        });
}

fn handle_stranger_actions(
    mut stranger_query: Query<
        (
            &mut KinematicCharacterController,
            &ActionState<StrangerAction>,
        ),
        With<Stranger>,
    >,
    mut camera_query: Query<&mut Transform, With<InGameCamera>>,
    window_query: Query<&Window>,
) {
    let mut camera_transform = camera_query.single_mut();
    for (mut stranger_character_controller, action_state) in stranger_query.iter_mut() {
        if action_state.pressed(StrangerAction::MoveForward) {
            info!("Stranger moved forward!");
        }

        if action_state.pressed(StrangerAction::MoveBackward) {
            info!("Stranger moved backward!");
        }

        if action_state.pressed(StrangerAction::MoveLeft) {
            info!("Stranger moved left!");
        }

        if action_state.pressed(StrangerAction::MoveRight) {
            info!("Stranger moved right!");
        }

        let camera_rotate_vector = action_state
            .axis_pair(StrangerAction::LookAround)
            .expect("Unable to get camera rotate vector");

        if camera_rotate_vector.length_squared() > 0. {
            let window = window_query.get_single().expect("Unable to get window");
            let delta_x = camera_rotate_vector.x() / window.width() * std::f32::consts::PI * 2.;
            let delta_y = camera_rotate_vector.y() / window.height() * std::f32::consts::PI;

            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);
            camera_transform.rotation = yaw * camera_transform.rotation;
            camera_transform.rotation = camera_transform.rotation * pitch;
        }
    }
}
