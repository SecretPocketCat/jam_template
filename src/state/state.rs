use bevy::prelude::*;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    Loading,
    Playing,
    Menu,
}

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum PauseState {
    #[default]
    Unpaused,
    Paused,
}
