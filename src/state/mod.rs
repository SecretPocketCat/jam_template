use bevy::prelude::*;
use seldom_fn_plugin::FnPluginExt;
pub use state::AppState;
pub use state::GameState;

mod pause;
mod state;

pub fn state_plugin(app: &mut App) {
    app.fn_plugin(pause::pause_plugin);
}
