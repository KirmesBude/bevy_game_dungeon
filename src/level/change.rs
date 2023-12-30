use bevy::prelude::*;
use bevy_flycam::FlyCam;

use crate::{
    controls::Controllable,
    loading::LevelAssets,
    movement::{GridDirection, GridPosition},
};

use super::{Level, Player};

#[derive(Debug, Event)]
pub struct ChangeLevel {
    pub level: Handle<Level>,
    pub position: Option<GridPosition>,
}

pub fn setup(
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

    change_level_evw.send(ChangeLevel {
        level: level_assets.levels.get("level/000.lvl").unwrap().clone(),
        position: None,
    });
}
