mod asset;
mod create;

use bevy::prelude::*;
use serde::Deserialize;

use crate::{loading::LevelAssets, GameState};

use self::{
    asset::LevelAssetLoader,
    create::{level_change_create, level_change_despawn, TileMaterials, TileMesh},
};

pub use asset::Level;
pub use create::{ChangeLevel, CurrentLevel};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentLevel>()
            .init_resource::<TileMesh>()
            .init_resource::<TileMaterials>()
            .init_asset::<Level>()
            .init_asset_loader::<LevelAssetLoader>()
            .add_event::<ChangeLevel>()
            .add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(
                Update,
                (level_change_despawn, level_change_create)
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

const TILE_SIZE: f32 = 32.0;

#[derive(Debug, Default, Deserialize, Eq, Hash, PartialEq)]
pub enum Tile {
    #[default]
    Void,
    Stone,
}

fn setup(
    mut commands: Commands,
    mut change_level_evw: EventWriter<ChangeLevel>,
    level_assets: Res<LevelAssets>,
) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 6., 12.0).looking_at(Vec3::new(32., 1., 32.), Vec3::Y),
        ..default()
    });

    change_level_evw.send(ChangeLevel(level_assets.start.clone()));
}
