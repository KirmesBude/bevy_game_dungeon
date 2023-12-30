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
        .add_collection_to_loading_state::<_, SceneAssets>(GameState::Loading)
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
pub struct SceneAssets {
    #[asset(path = "models/KayKit_DungeonRemastered_1.0_FREE/chest.glb#Scene0")]
    chest: Handle<Scene>,
    #[asset(path = "models/KayKit_DungeonRemastered_1.0_FREE/floor_tile_large.gltf.glb#Scene0")]
    floor_tile: Handle<Scene>,
    #[asset(path = "models/KayKit_DungeonRemastered_1.0_FREE/key.gltf.glb#Scene0")]
    _key: Handle<Scene>,
    #[asset(path = "models/KayKit_DungeonRemastered_1.0_FREE/torch_mounted.gltf.glb#Scene0")]
    _torch: Handle<Scene>,
    #[asset(path = "models/KayKit_DungeonRemastered_1.0_FREE/wall.gltf.glb#Scene0")]
    wall: Handle<Scene>,
    #[asset(path = "models/KayKit_DungeonRemastered_1.0_FREE/wall_doorway.glb#Scene0")]
    _door: Handle<Scene>,
}

impl SceneAssets {
    pub fn chest(&self, transform: Transform) -> SceneBundle {
        let mut transform = transform;
        transform.translation += Vec3::new(0.0, -16.0, -4.0);
        SceneBundle {
            scene: self.chest.clone(),
            transform: transform.with_scale(Vec3::splat(8.0)),
            ..Default::default()
        }
    }

    pub fn floor_tile(&self, transform: Transform) -> SceneBundle {
        SceneBundle {
            scene: self.floor_tile.clone(),
            transform: transform.with_scale(Vec3::splat(8.0)),
            ..Default::default()
        }
    }

    pub fn wall(&self, transform: Transform) -> SceneBundle {
        let mut transform = transform;
        transform.translation += Vec3::new(0.0, -16.0, 0.0);
        SceneBundle {
            scene: self.wall.clone(),
            transform: transform.with_scale(Vec3::splat(8.0)),
            ..Default::default()
        }
    }
}
