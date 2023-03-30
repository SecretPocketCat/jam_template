use bevy::prelude::*;
use seldom_fn_plugin::FnPluginExt;

pub mod command;
pub mod sequence;
mod state_machine;
pub mod transition;
pub mod trigger;

pub use state_machine::StateMachine;

pub fn state_machine_plugin(app: &mut App) {
    // todo plugins
    // app.fn_plugin(pause::pause_plugin);
}
