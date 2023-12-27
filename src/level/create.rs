use bevy::prelude::*;

use crate::loading::TileTextureAssets;

use super::{asset::Level, Player, Tile, TILE_SIZE};

/// Holds a Handle to a Level Asset of the currently loaded level
#[derive(Debug, Default, Resource, Deref)]
pub struct CurrentLevel(Handle<Level>);

/// Holds a Handle to a Level Asset of the currently loaded level
#[derive(Debug, Resource, Deref)]
pub struct TileMesh(Handle<Mesh>);

impl FromWorld for TileMesh {
    fn from_world(world: &mut World) -> Self {
        let mut meshes: Mut<Assets<Mesh>> = world.get_resource_mut().unwrap();

        TileMesh(meshes.add(shape::Plane::from_size(TILE_SIZE).into()))
    }
}

/// Marker Component so all level specific entities can be despawned of level change
#[derive(Debug, Default, Component)]
pub struct LevelGeometry;

fn create_level_geometry(
    commands: &mut Commands,
    level: &Level,
    tile_mesh: &Handle<Mesh>,
    tile_textures: &TileTextureAssets,
) {
    for (y, row) in level.grid.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            match tile {
                Tile::Void => { /* do nothing */ }
                tile => {
                    let material = tile_textures.get(tile).unwrap();

                    /* Ground */
                    let translation =
                        Vec3::new(x as f32 * TILE_SIZE, -TILE_SIZE / 2.0, y as f32 * TILE_SIZE);
                    commands
                        .spawn(PbrBundle {
                            mesh: tile_mesh.clone(),
                            material: material.clone(),
                            transform: Transform::from_translation(translation),
                            ..default()
                        })
                        .insert(LevelGeometry);

                    /* North Wall */
                    if (y == 0) || matches!(level.grid[y - 1][x], Tile::Void) {
                        let translation =
                            translation + Vec3::new(0.0, TILE_SIZE / 2.0, -TILE_SIZE / 2.0);

                        commands
                            .spawn(PbrBundle {
                                mesh: tile_mesh.clone(),
                                material: material.clone(),
                                transform: Transform::from_translation(translation)
                                    .looking_to(Vec3::Y, Vec3::Z),
                                ..default()
                            })
                            .insert(LevelGeometry);
                    }
                    /* South Wall */
                    if (y == level.grid.len() - 1) || matches!(level.grid[y + 1][x], Tile::Void) {
                        let translation =
                            translation + Vec3::new(0.0, TILE_SIZE / 2.0, TILE_SIZE / 2.0);

                        commands
                            .spawn(PbrBundle {
                                mesh: tile_mesh.clone(),
                                material: material.clone(),
                                transform: Transform::from_translation(translation)
                                    .looking_to(Vec3::Y, -Vec3::Z),
                                ..default()
                            })
                            .insert(LevelGeometry);
                    }
                    /* West Wall */
                    if (x == 0) || matches!(level.grid[y][x - 1], Tile::Void) {
                        let translation =
                            translation + Vec3::new(-TILE_SIZE / 2.0, TILE_SIZE / 2.0, 0.0);

                        commands
                            .spawn(PbrBundle {
                                mesh: tile_mesh.clone(),
                                material: material.clone(),
                                transform: Transform::from_translation(translation)
                                    .looking_to(Vec3::Y, Vec3::X),
                                ..default()
                            })
                            .insert(LevelGeometry);
                    }
                    /* East Wall */
                    if (x == level.grid[y].len() - 1) || matches!(level.grid[y][x + 1], Tile::Void)
                    {
                        let translation =
                            translation + Vec3::new(TILE_SIZE / 2.0, TILE_SIZE / 2.0, 0.0);

                        commands
                            .spawn(PbrBundle {
                                mesh: tile_mesh.clone(),
                                material: material.clone(),
                                transform: Transform::from_translation(translation)
                                    .looking_to(Vec3::Y, -Vec3::X),
                                ..default()
                            })
                            .insert(LevelGeometry);
                    }

                    /* Ceiling */
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

#[derive(Debug, Event)]
pub struct ChangeLevel(pub Handle<Level>);

pub fn level_change_despawn(
    mut commands: Commands,
    mut change_level_evr: EventReader<ChangeLevel>,
    level_entities: Query<Entity, With<LevelGeometry>>,
) {
    for _ in change_level_evr.read() {
        despawn_level_geometry(&mut commands, &level_entities);
    }
}

pub fn level_change_create(
    mut commands: Commands,
    mut change_level_evr: EventReader<ChangeLevel>,
    mut current_level: ResMut<CurrentLevel>,
    tile_mesh: Res<TileMesh>,
    tile_textures: Res<TileTextureAssets>,
    level_assets: Res<Assets<Level>>,
) {
    for event in change_level_evr.read() {
        current_level.0 = event.0.clone();

        let level = level_assets.get(event.0.clone()).unwrap();

        create_level_geometry(&mut commands, level, &tile_mesh.0, &tile_textures);
    }
}

pub fn move_player_to_start_pos(
    mut change_level_evr: EventReader<ChangeLevel>,
    level_assets: Res<Assets<Level>>,
    mut player_pos: Query<&mut Transform, With<Player>>,
) {
    for event in change_level_evr.read() {
        let level = level_assets.get(event.0.clone()).unwrap();

        for mut transform in &mut player_pos {
            let translation = Vec3::new(
                level.start_pos.0 as f32 * TILE_SIZE,
                0.0,
                level.start_pos.1 as f32 * TILE_SIZE,
            );

            transform.translation = translation;
            transform.look_to(Vec3::Z, Vec3::Y);
        }
    }
}
