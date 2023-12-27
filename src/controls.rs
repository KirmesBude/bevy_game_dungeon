use bevy::prelude::*;

use crate::{
    movement::{FaceDirection, GridDirection, GridPosition, MoveForward},
    GameState,
};

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (move_forwards_controls, face_direction_controls)
                .chain()
                .run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Debug, Default, Component)]
pub struct Controllable;

fn face_direction_controls(
    key_input: Res<Input<KeyCode>>,
    directions: Query<(Entity, &GridDirection), With<Controllable>>,
    mut face_direction_evw: EventWriter<FaceDirection>,
) {
    for (entity, direction) in &directions {
        if key_input.just_pressed(KeyCode::Right) {
            face_direction_evw.send(FaceDirection {
                entity,
                direction: direction.right(),
            });
        } else if key_input.just_pressed(KeyCode::Down) {
            face_direction_evw.send(FaceDirection {
                entity,
                direction: direction.back(),
            });
        } else if key_input.just_pressed(KeyCode::Left) {
            face_direction_evw.send(FaceDirection {
                entity,
                direction: direction.left(),
            });
        }
    }
}

fn move_forwards_controls(
    key_input: Res<Input<KeyCode>>,
    grid_positions: Query<Entity, (With<Controllable>, With<GridPosition>)>,
    mut move_forward_evw: EventWriter<MoveForward>,
) {
    for entity in &grid_positions {
        if key_input.just_pressed(KeyCode::Up) {
            move_forward_evw.send(MoveForward { entity });
        }
    }
}
