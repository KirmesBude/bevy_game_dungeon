use std::num::TryFromIntError;

use bevy::prelude::*;

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
                (grid_position_to_translation, direction_to_rotation)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Default, Debug, Component, Clone, Copy)]
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

fn move_forward(
    mut move_forward_evr: EventReader<MoveForward>,
    mut query: Query<(&mut GridPosition, &GridDirection)>,
    current_level: Res<CurrentLevel>,
    level_assets: Res<Assets<Level>>,
) {
    /* TODO: Other stuff occupying the position need to be checked */
    if let Some(level) = level_assets.get(&current_level.0) {
        for event in move_forward_evr.read() {
            if let Ok((mut grid_position, direction)) = query.get_mut(event.entity) {
                if let Ok(next_position) = grid_position.next(direction) {
                    /* Check the outer boundaries */
                    if next_position.y < level.grid.len()
                        && next_position.x < level.grid[next_position.y].len()
                    {
                        /* Check for void */
                        if !matches!(level.grid[next_position.y][next_position.x], Tile::Void) {
                            *grid_position = next_position;
                        }
                    }
                }
            }
        }
    }
}

fn face_direction(
    mut face_direction_evr: EventReader<FaceDirection>,
    mut directions: Query<&mut GridDirection>,
) {
    for event in face_direction_evr.read() {
        if let Ok(mut direction) = directions.get_mut(event.entity) {
            *direction = event.direction;
        }
    }
}
