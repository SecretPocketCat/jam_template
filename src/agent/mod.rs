use bevy::prelude::App;

use self::agent::{move_agent, wrap_agent};

pub mod agent;

pub fn agent_plugin(app: &mut App) {
    // todo: schedule before trans propagation?
    app.add_system(move_agent).add_system(wrap_agent);
}
