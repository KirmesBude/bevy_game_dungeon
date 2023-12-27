use crate::{level::Level, GameState};
use bevy::{prelude::*, utils::HashMap};
use bevy_asset_loader::prelude::*;

pub struct LoadingPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Menu),
        )
        .add_collection_to_loading_state::<_, TextureAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, TileTextureAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, LevelAssets>(GameState::Loading);
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(Debug, AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "bevy.png")]
    pub bevy: Handle<Image>,
    #[asset(path = "github.png")]
    pub github: Handle<Image>,
}

#[derive(Debug, AssetCollection, Resource)]
pub struct LevelAssets {
    #[asset(path = "level", collection(typed, mapped))]
    pub levels: HashMap<String, Handle<Level>>,
}

#[derive(Debug, AssetCollection, Resource)]
pub struct TileTextureAssets {
    #[asset(standard_material)]
    #[asset(path = "stone.png")]
    pub stone: Handle<StandardMaterial>,
}
