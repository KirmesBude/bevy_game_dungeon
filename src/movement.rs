use std::{num::TryFromIntError, time::Duration};

use bevy::prelude::*;
use bevy_easings::{Ease, EaseFunction, EasingComponent, EasingType};
use serde::Deserialize;

use crate::{
    level::{CurrentLevel, Level, Tile, TILE_SIZE},
    GameState,
};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<FaceDirection>()
            .add_event::<MoveForward>()
            .add_systems(
                Update,
                (move_forward, face_direction).run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                (
                    ease_grid_position_to_translation,
                    ease_direction_to_rotation,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                (grid_position_to_translation, direction_to_rotation)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                PostUpdate,
                (end_of_ease::<GridPosition>, end_of_ease::<GridDirection>)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Default, Debug, Component, Clone, Copy, Deserialize, PartialEq, Eq, Hash)]
pub struct GridPosition {
    pub x: usize,
    pub y: usize,
}

impl From<&GridPosition> for Vec3 {
    fn from(grid_position: &GridPosition) -> Self {
        Vec3::new(
            grid_position.x as f32 * TILE_SIZE,
            0.0,
            grid_position.y as f32 * TILE_SIZE,
        )
    }
}

impl GridPosition {
    pub fn next(&self, direction: &GridDirection) -> Result<Self, TryFromIntError> {
        let delta = match direction {
            GridDirection::North => (0, -1),
            GridDirection::East => (1, 0),
            GridDirection::South => (0, 1),
            GridDirection::West => (-1, 0),
        };

        Ok(Self {
            x: usize::try_from(self.x as i32 + delta.0)?,
            y: usize::try_from(self.y as i32 + delta.1)?,
        })
    }
}

#[derive(Default, Debug, Component, Clone, Copy)]
pub enum GridDirection {
    North,
    East,
    #[default]
    South,
    West,
}

impl From<&GridDirection> for Vec3 {
    fn from(direction: &GridDirection) -> Self {
        match direction {
            GridDirection::North => -Vec3::Z,
            GridDirection::East => Vec3::X,
            GridDirection::South => Vec3::Z,
            GridDirection::West => -Vec3::X,
        }
    }
}

impl From<GridDirection> for Vec3 {
    fn from(direction: GridDirection) -> Self {
        (&direction).into()
    }
}

impl GridDirection {
    pub fn front(&self) -> Self {
        *self
    }

    pub fn right(&self) -> Self {
        match self {
            GridDirection::North => GridDirection::East,
            GridDirection::East => GridDirection::South,
            GridDirection::South => GridDirection::West,
            GridDirection::West => GridDirection::North,
        }
    }

    pub fn back(&self) -> Self {
        match self {
            GridDirection::North => GridDirection::South,
            GridDirection::East => GridDirection::West,
            GridDirection::South => GridDirection::North,
            GridDirection::West => GridDirection::East,
        }
    }

    pub fn left(&self) -> Self {
        match self {
            GridDirection::North => GridDirection::West,
            GridDirection::East => GridDirection::North,
            GridDirection::South => GridDirection::East,
            GridDirection::West => GridDirection::South,
        }
    }
}

#[derive(Debug, Event)]
pub struct FaceDirection {
    pub entity: Entity,
    pub direction: GridDirection,
}

#[derive(Debug, Event)]
pub struct MoveForward {
    pub entity: Entity,
}

#[derive(Debug, Component)]
pub struct EaseTo<T: Copy> {
    pub target: T,
}

impl<T: Copy> EaseTo<T> {
    pub fn new(target: T) -> Self {
        Self { target }
    }
}

fn ease_grid_position_to_translation(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &EaseTo<GridPosition>), Added<EaseTo<GridPosition>>>,
) {
    for (entity, transform, grid_position) in &query {
        let new_grid_position = &grid_position.target;
        let new_transform = transform.with_translation(new_grid_position.into());

        commands.entity(entity).insert(transform.ease_to(
            new_transform,
            EaseFunction::SineInOut,
            EasingType::Once {
                duration: Duration::from_millis(300),
            },
        ));
    }
}

fn ease_direction_to_rotation(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &EaseTo<GridDirection>), Added<EaseTo<GridDirection>>>,
) {
    for (entity, transform, direction) in &query {
        let new_direction = &direction.target;
        let new_transform = transform.looking_to(new_direction.into(), Vec3::Y);

        commands.entity(entity).insert(transform.ease_to(
            new_transform,
            EaseFunction::SineInOut,
            EasingType::Once {
                duration: Duration::from_millis(300),
            },
        ));
    }
}

fn move_forward(
    mut commands: Commands,
    mut move_forward_evr: EventReader<MoveForward>,
    query: Query<(&GridPosition, &GridDirection), Without<EasingComponent<Transform>>>,
    current_level: Res<CurrentLevel>,
    level_assets: Res<Assets<Level>>,
) {
    /* TODO: Other stuff occupying the position need to be checked */
    if let Some(level) = level_assets.get(&current_level.0) {
        for event in move_forward_evr.read() {
            if let Ok((grid_position, direction)) = query.get(event.entity) {
                if let Ok(next_position) = grid_position.next(direction) {
                    /* Check the outer boundaries */
                    if next_position.y < level.grid.len()
                        && next_position.x < level.grid[next_position.y].len()
                    {
                        /* Check for void */
                        if !matches!(level.grid[next_position.y][next_position.x], Tile::Void) {
                            /* Check for Interactables */
                            if !level.interactables.contains_key(&next_position) {
                                commands
                                    .entity(event.entity)
                                    .insert(EaseTo::new(next_position));
                            }
                        }
                    }
                }
            }
        }
    }
}

fn face_direction(
    mut commands: Commands,
    mut face_direction_evr: EventReader<FaceDirection>,
    directions: Query<&GridDirection, Without<EasingComponent<Transform>>>,
) {
    for event in face_direction_evr.read() {
        if let Ok(_direction) = directions.get(event.entity) {
            commands
                .entity(event.entity)
                .insert(EaseTo::new(event.direction));
        }
    }
}

fn grid_position_to_translation(
    mut query: Query<(&mut Transform, &GridPosition), Changed<GridPosition>>,
) {
    for (mut transform, grid_position) in &mut query {
        transform.translation = grid_position.into();
    }
}

fn direction_to_rotation(
    mut query: Query<(&mut Transform, &GridDirection), Changed<GridDirection>>,
) {
    for (mut transform, direction) in &mut query {
        transform.look_to(direction.into(), Vec3::Y);
    }
}

fn end_of_ease<T: Component + Copy>(
    mut commands: Commands,
    mut removed: RemovedComponents<EasingComponent<Transform>>,
    mut query: Query<(&mut T, &EaseTo<T>)>,
) {
    for entity in removed.read() {
        if let Ok((mut component, ease_to)) = query.get_mut(entity) {
            *component = ease_to.target;

            commands.entity(entity).remove::<EaseTo<T>>();
        }
    }
}
