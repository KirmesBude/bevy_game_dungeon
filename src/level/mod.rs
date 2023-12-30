mod asset;
mod change;
mod create;

use bevy::prelude::*;
use serde::Deserialize;

use crate::GameState;

use self::{
    asset::LevelAssetLoader,
    change::{change_level, setup, ChangeLevel},
    create::{level_change_create, level_change_despawn, move_player_to_start_pos},
};

pub use asset::Level;

/// Holds a Handle to a Level Asset of the currently loaded level
#[derive(Debug, Default, Resource, Deref)]
pub struct CurrentLevel(pub Handle<Level>);

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentLevel>()
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
            )
            .add_systems(
                Update,
                change_level
                    .before(level_change_despawn)
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

/* TODO: Move somewhere else */
#[derive(Debug, Default, Component)]
pub struct Player;
