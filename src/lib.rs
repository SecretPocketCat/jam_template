#![feature(let_chains)]

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use seldom_fn_plugin::FnPluginExt;

use state::GameState;

mod animation;
mod assets;
mod audio;
mod input;
mod io;
mod player;
mod render;
mod state;
mod time;
mod tools;
mod ui;

pub const GAME_NAME: &str = "todo";

pub fn game_plugin(app: &mut App) {
    app.add_state::<GameState>()
        .fn_plugin(animation::animation_plugin)
        .fn_plugin(assets::assets_plugin)
        .fn_plugin(audio::audio_plugin)
        .fn_plugin(render::render_plugin)
        .fn_plugin(ui::ui_plugin)
        .fn_plugin(input::input_plugin)
        .fn_plugin(player::player_plugin)
        .fn_plugin(time::time_plugin)
        .fn_plugin(io::io_plugin);

    #[cfg(debug_assertions)]
    {
        // app.add_plugin(FrameTimeDiagnosticsPlugin::default())
        //     .add_plugin(LogDiagnosticsPlugin::default());
    }
}
