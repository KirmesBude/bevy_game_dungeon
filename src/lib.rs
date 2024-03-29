#![allow(clippy::type_complexity)]

mod controls;
mod level;
mod loading;
mod menu;
mod movement;

use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
#[cfg(debug_assertions)]
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_easings::EasingsPlugin;
#[cfg(debug_assertions)]
use bevy_flycam::NoCameraPlayerPlugin;
#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use controls::ControlsPlugin;
use level::LevelPlugin;
use movement::MovementPlugin;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_plugins((
            LoadingPlugin,
            MenuPlugin,
            LevelPlugin,
            MovementPlugin,
            ControlsPlugin,
            EasingsPlugin,
        ));

        #[cfg(debug_assertions)]
        {
            app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
            app.add_plugins(
                WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::KeyI)),
            );
            app.add_plugins(NoCameraPlayerPlugin);
        }
    }
}
