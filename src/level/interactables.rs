use bevy::prelude::*;
use serde::Deserialize;

use crate::{
    loading::{LevelAssets, SceneAssets},
    movement::GridPosition,
    GameState,
};

use super::change::ChangeLevel;

pub struct InteractablePlugin;

impl Plugin for InteractablePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Interact>()
            .add_systems(Update, interact.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Debug, Event)]
pub struct Interact {
    pub source: Entity,
    pub target: Interactable,
}

#[derive(Debug, Component, Deserialize, Clone)]
pub enum Interactable {
    Chest(Loot),
    Door,
    Teleporter(Teleporter),
}

impl Interactable {
    pub fn bundle(&self, scene_assets: &SceneAssets, transform: Transform) -> SceneBundle {
        match self {
            Interactable::Chest(_) => scene_assets.chest(transform),
            Interactable::Door => todo!(),
            Interactable::Teleporter(_) => scene_assets.pillar(transform),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub enum Loot {
    Key,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Teleporter {
    grid_position: GridPosition,
    #[serde(default)]
    level: Option<String>,
}

pub fn interact(
    mut interact_evr: EventReader<Interact>,
    mut change_level_evw: EventWriter<ChangeLevel>,
    level_assets: Res<LevelAssets>,
    mut positions: Query<&mut GridPosition>,
) {
    for event in interact_evr.read() {
        match &event.target {
            Interactable::Chest(loot) => info!("Interact with Chest to get loot: {:?}", loot),
            Interactable::Door => todo!(),
            Interactable::Teleporter(teleporter) => {
                info!("Interact with Teleporter{:?}", teleporter);
                match &teleporter.level {
                    Some(level_name) => {
                        change_level_evw.send(ChangeLevel {
                            level: level_assets.levels.get(level_name).unwrap().clone(),
                            position: Some(teleporter.grid_position),
                        });
                    }
                    None => {
                        if let Ok(mut position) = positions.get_mut(event.source) {
                            *position = teleporter.grid_position;
                        }
                    }
                }
            }
        }
    }
}
