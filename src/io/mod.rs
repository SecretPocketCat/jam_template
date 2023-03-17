use bevy::prelude::*;

use save::{load_game, save_game};

pub mod save;

pub fn io_plugin(app: &mut App) {
    app.add_system(save_game).add_startup_system(load_game);
}
