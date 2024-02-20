use bevy::prelude::*;

use crate::{
    level::{CurrentLevel, Interact, Level},
    movement::{FaceDirection, GridDirection, GridPosition, MoveForward},
    GameState,
};

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                move_forwards_controls,
                face_direction_controls,
                interact_controls,
            )
                .chain()
                .run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Debug, Default, Component)]
pub struct Controllable;

fn face_direction_controls(
    key_input: Res<ButtonInput<KeyCode>>,
    directions: Query<(Entity, &GridDirection), With<Controllable>>,
    mut face_direction_evw: EventWriter<FaceDirection>,
) {
    for (entity, direction) in &directions {
        if key_input.just_pressed(KeyCode::ArrowRight) {
            face_direction_evw.send(FaceDirection {
                entity,
                direction: direction.right(),
            });
        } else if key_input.just_pressed(KeyCode::ArrowDown) {
            face_direction_evw.send(FaceDirection {
                entity,
                direction: direction.back(),
            });
        } else if key_input.just_pressed(KeyCode::ArrowLeft) {
            face_direction_evw.send(FaceDirection {
                entity,
                direction: direction.left(),
            });
        }
    }
}

fn move_forwards_controls(
    key_input: Res<ButtonInput<KeyCode>>,
    grid_positions: Query<Entity, (With<Controllable>, With<GridPosition>)>,
    mut move_forward_evw: EventWriter<MoveForward>,
) {
    for entity in &grid_positions {
        if key_input.just_pressed(KeyCode::ArrowUp) {
            move_forward_evw.send(MoveForward { entity });
        }
    }
}

/* TODO: I would much rather get the entity here and send that in the Interact event */
fn interact_controls(
    key_input: Res<ButtonInput<KeyCode>>,
    controllables: Query<(Entity, &GridPosition, &GridDirection), With<Controllable>>,
    current_level: Res<CurrentLevel>,
    level_assets: Res<Assets<Level>>,
    mut interact_evw: EventWriter<Interact>,
) {
    for (entity, grid_position, direction) in &controllables {
        if key_input.just_pressed(KeyCode::Space) {
            if let Ok(interact_position) = grid_position.next(direction) {
                if let Some(level) = level_assets.get(&current_level.0) {
                    if let Some(interactable) = level.interactables.get(&interact_position) {
                        interact_evw.send(Interact {
                            source: entity,
                            target: interactable.clone(),
                        });
                    }
                }
            }
        }
    }
}
