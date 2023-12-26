mod asset;
mod create;

use bevy::prelude::*;
use bevy_flycam::FlyCam;
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
            SpatialBundle {
                transform: Transform::from_xyz(0.0, 6., 12.0)
                    .looking_at(Vec3::new(32., 1., 32.), Vec3::Y),
                ..default()
            },
            FlyCam,
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
