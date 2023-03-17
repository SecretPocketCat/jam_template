use bevy::prelude::*;

use self::player::{move_player, spawn_player};
use crate::GameState;

pub mod player;

pub fn player_plugin(app: &mut App) {
    app.add_system(spawn_player.in_schedule(OnEnter(GameState::Playing)))
        .add_system(move_player.in_set(OnUpdate(GameState::Playing)));
}
