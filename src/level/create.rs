use bevy::{prelude::*, utils::HashMap};

use super::{asset::Level, Tile, TILE_SIZE};

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

/// Holds a Handle to a Level Asset of the currently loaded level
#[derive(Debug, Resource, Deref)]
pub struct TileMaterials(HashMap<Tile, Handle<StandardMaterial>>);

impl FromWorld for TileMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials: Mut<Assets<StandardMaterial>> = world.get_resource_mut().unwrap();
        let mut hash_map = HashMap::new();

        hash_map.insert(Tile::Void, materials.add(Color::BLACK.into()));
        hash_map.insert(Tile::Stone, materials.add(Color::ORANGE_RED.into()));

        TileMaterials(hash_map)
    }
}

/// Marker Component so all level specific entities can be despawned of level change
#[derive(Debug, Default, Component)]
pub struct LevelGeometry;

fn create_level_geometry(
    commands: &mut Commands,
    level: &Level,
    tile_mesh: &Handle<Mesh>,
    tile_materials: &HashMap<Tile, Handle<StandardMaterial>>,
) {
    for (y, row) in level.grid.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            let translation =
                Vec3::new(x as f32 * TILE_SIZE, -TILE_SIZE / 2.0, y as f32 * TILE_SIZE);

            commands
                .spawn(PbrBundle {
                    mesh: tile_mesh.clone(),
                    material: tile_materials.get(tile).unwrap().clone(),
                    transform: Transform::from_translation(translation),
                    ..default()
                })
                .insert(LevelGeometry);
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
    tile_materials: Res<TileMaterials>,
    level_assets: Res<Assets<Level>>,
) {
    for event in change_level_evr.read() {
        current_level.0 = event.0.clone();

        let level = level_assets.get(event.0.clone()).unwrap();

        create_level_geometry(&mut commands, level, &tile_mesh.0, &tile_materials.0);
    }
}
