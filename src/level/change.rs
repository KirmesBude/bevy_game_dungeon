use bevy::prelude::*;
use bevy_easings::EasingComponent;
use bevy_flycam::FlyCam;

use crate::{
    controls::Controllable,
    loading::{LevelAssets, SceneAssets},
    movement::{GridDirection, GridPosition},
};

use super::{CurrentLevel, Level, Player};

#[derive(Debug, Event)]
pub struct ChangeLevel(pub Handle<Level>);

pub fn setup(
    mut commands: Commands,
    mut change_level_evw: EventWriter<ChangeLevel>,
    level_assets: Res<LevelAssets>,
    scene_assets: Res<SceneAssets>,
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

    change_level_evw.send(ChangeLevel(
        level_assets.levels.get("level/000.lvl").unwrap().clone(),
    ));

    commands.spawn(scene_assets.chest());
}

pub fn change_level(
    mut change_level_evw: EventWriter<ChangeLevel>,
    player_pos: Query<&GridPosition, (With<Player>, Without<EasingComponent<Transform>>)>,
    current_level: Res<CurrentLevel>,
    level_assets: Res<Assets<Level>>,
    levels: Res<LevelAssets>,
) {
    if let Some(level) = level_assets.get(&current_level.0) {
        if let Some(next_level) = &level.next_level {
            for position in &player_pos {
                if position == &level.end_pos {
                    change_level_evw
                        .send(ChangeLevel(levels.levels.get(next_level).unwrap().clone()));
                    return;
                }
            }
        }
    }
}
