use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub(super) fn actions_plugin(app: &mut App) {
    app.add_plugin(InputManagerPlugin::<PlayerAction>::default())
        .add_plugin(InputManagerPlugin::<UiAction>::default());
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum PlayerAction {
    Move,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum UiAction {
    Move,
    Confirm,
    Cancel,
}
