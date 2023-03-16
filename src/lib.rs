#![feature(let_chains)]

mod actions;
mod audio;
mod camera;
mod loading;
mod menu;
mod mouse;
mod player;
mod tools;
mod tween;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_tweening::TweeningPlugin;
use seldom_fn_plugin::FnPluginExt;

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

pub fn game_plugin(app: &mut App) {
    app.add_state::<GameState>()
        .fn_plugin(loading::loading_plugin)
        .fn_plugin(camera::camera_plugin)
        .fn_plugin(menu::menu_plugin)
        .fn_plugin(actions::actions_plugin)
        .fn_plugin(audio::audio_plugin)
        .fn_plugin(tween::game_tween_plugin)
        .fn_plugin(mouse::mouse_plugin)
        .fn_plugin(player::player_plugin)
        .add_plugin(TweeningPlugin);

    #[cfg(debug_assertions)]
    {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(LogDiagnosticsPlugin::default());
    }
}
