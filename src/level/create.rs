use bevy::prelude::*;

use crate::{
    loading::SceneAssets,
    movement::{GridDirection, GridPosition},
};

use super::{asset::Level, change::ChangeLevel, CurrentLevel, Player, Tile, TILE_SIZE};

/// Marker Component so all level specific entities can be despawned of level change
#[derive(Debug, Default, Component)]
pub struct LevelGeometry;

fn create_level_geometry(commands: &mut Commands, level: &Level, scene_assets: &SceneAssets) {
    for (y, row) in level.grid.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            match tile {
                Tile::Void => { /* do nothing */ }
                _tile => {
                    /* Ground */
                    let translation =
                        Vec3::new(x as f32 * TILE_SIZE, -TILE_SIZE / 2.0, y as f32 * TILE_SIZE);
                    commands
                        .spawn(scene_assets.floor_tile(Transform::from_translation(translation)))
                        .insert(LevelGeometry);

                    /* North Wall */
                    if (y == 0) || matches!(level.grid[y - 1][x], Tile::Void) {
                        let translation =
                            translation + Vec3::new(0.0, TILE_SIZE / 2.0, -TILE_SIZE / 2.0);

                        commands
                            .spawn(
                                scene_assets.wall(
                                    Transform::from_translation(translation)
                                        .looking_to(GridDirection::North.into(), Vec3::Y),
                                ),
                            )
                            .insert(LevelGeometry);
                    }
                    /* South Wall */
                    if (y == level.grid.len() - 1) || matches!(level.grid[y + 1][x], Tile::Void) {
                        let translation =
                            translation + Vec3::new(0.0, TILE_SIZE / 2.0, TILE_SIZE / 2.0);

                        commands
                            .spawn(
                                scene_assets.wall(
                                    Transform::from_translation(translation)
                                        .looking_to(GridDirection::South.into(), Vec3::Y),
                                ),
                            )
                            .insert(LevelGeometry);
                    }
                    /* West Wall */
                    if (x == 0) || matches!(level.grid[y][x - 1], Tile::Void) {
                        let translation =
                            translation + Vec3::new(-TILE_SIZE / 2.0, TILE_SIZE / 2.0, 0.0);

                        commands
                            .spawn(
                                scene_assets.wall(
                                    Transform::from_translation(translation)
                                        .looking_to(GridDirection::West.into(), Vec3::Y),
                                ),
                            )
                            .insert(LevelGeometry);
                    }
                    /* East Wall */
                    if (x == level.grid[y].len() - 1) || matches!(level.grid[y][x + 1], Tile::Void)
                    {
                        let translation =
                            translation + Vec3::new(TILE_SIZE / 2.0, TILE_SIZE / 2.0, 0.0);

                        commands
                            .spawn(
                                scene_assets.wall(
                                    Transform::from_translation(translation)
                                        .looking_to(GridDirection::East.into(), Vec3::Y),
                                ),
                            )
                            .insert(LevelGeometry);
                    }

                    /* Ceiling */
                    /*
                    let translation =
                        Vec3::new(x as f32 * TILE_SIZE, TILE_SIZE / 2.0, y as f32 * TILE_SIZE);
                    commands
                        .spawn(PbrBundle {
                            mesh: tile_mesh.clone(),
                            material: material.clone(),
                            transform: Transform::from_translation(translation).looking_to(-Vec3::X, -Vec3::Y), /* All of these rotations make no sense */
                            ..default()
                        })
                        .insert(LevelGeometry);
                    */
                }
            }
        }
    }
}

fn despawn_level_geometry(commands: &mut Commands, entities: &Query<Entity, With<LevelGeometry>>) {
    for entity in entities {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn level_change_despawn(
    mut commands: Commands,
    mut change_level_evr: EventReader<ChangeLevel>,
    level_entities: Query<Entity, With<LevelGeometry>>,
) {
    for _ in change_level_evr.read() {
        info!("Despawning...");
        despawn_level_geometry(&mut commands, &level_entities);
    }
}

pub fn level_change_create(
    mut commands: Commands,
    mut change_level_evr: EventReader<ChangeLevel>,
    mut current_level: ResMut<CurrentLevel>,
    level_assets: Res<Assets<Level>>,
    scene_assets: Res<SceneAssets>,
) {
    for event in change_level_evr.read() {
        info!("Creating...");
        current_level.0 = event.0.clone();

        let level = level_assets.get(&event.0).unwrap();

        create_level_geometry(&mut commands, level, &scene_assets);
    }
}

pub fn move_player_to_start_pos(
    mut change_level_evr: EventReader<ChangeLevel>,
    level_assets: Res<Assets<Level>>,
    mut player_pos: Query<(&mut GridPosition, &mut GridDirection), With<Player>>,
) {
    for event in change_level_evr.read() {
        info!("Moving player to start...");
        let level = level_assets.get(&event.0).unwrap();

        for (mut grid_position, mut grid_direction) in &mut player_pos {
            *grid_position = level.start_pos;
            *grid_direction = GridDirection::default();
        }
    }
}
