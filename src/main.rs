// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_game_dungeon::GamePlugin;

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(AssetMetaCheck::Never)
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Game Dungeon".to_string(),
                // Bind to canvas included in `index.html`
                canvas: Some("#bevy".to_owned()),
                // The canvas size is constrained in index.html and build/web/styles.css
                fit_canvas_to_parent: true,
                // Tells wasm not to override default event handling, like F5 and Ctrl+R
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(GamePlugin)
        .run();
}
