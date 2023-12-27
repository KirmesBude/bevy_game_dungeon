mod asset;
mod create;

use bevy::prelude::*;
use bevy_flycam::FlyCam;
use serde::Deserialize;

use crate::{
    controls::Controllable,
    loading::{LevelAssets, TileTextureAssets},
    movement::{GridDirection, GridPosition},
    GameState,
};

use self::{
    asset::LevelAssetLoader,
    create::{level_change_create, level_change_despawn, move_player_to_start_pos, TileMesh},
};

pub use asset::Level;
pub use create::{ChangeLevel, CurrentLevel};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentLevel>()
            .init_resource::<TileMesh>()
            .init_asset::<Level>()
            .init_asset_loader::<LevelAssetLoader>()
            .add_event::<ChangeLevel>()
            .add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(
                Update,
                (
                    level_change_despawn,
                    level_change_create,
                    move_player_to_start_pos,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

pub const TILE_SIZE: f32 = 32.0;

#[derive(Debug, Default, Deserialize, Eq, Hash, PartialEq)]
pub enum Tile {
    #[default]
    Void,
    Stone,
}

impl TileTextureAssets {
    pub fn get(&self, tile: &Tile) -> Option<Handle<StandardMaterial>> {
        match tile {
            Tile::Stone => Some(self.stone.clone()),
            Tile::Void => None,
        }
    }
}

/* TODO: Move somewhere else */
#[derive(Debug, Default, Component)]
pub struct Player;

fn setup(
    mut commands: Commands,
    mut change_level_evw: EventWriter<ChangeLevel>,
    level_assets: Res<LevelAssets>,
) {
    commands
        .spawn((
            Player,
            SpatialBundle::default(),
            FlyCam,
            Controllable,
            GridPosition::default(),
            GridDirection::default(),
        ))
        .with_children(|parent| {
            parent.spawn(PointLightBundle {
                point_light: PointLight {
                    color: Color::YELLOW,
                    intensity: 9000.0,
                    range: 64.0,
                    radius: 64.0,
                    shadows_enabled: true,
                    ..default()
                },
                ..default()
            });

            parent.spawn(Camera3dBundle::default());
        });

    change_level_evw.send(ChangeLevel(level_assets.start.clone()));
}
