use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use super::{state::reset_state, AppState, GameState};
use crate::input::actions::PlayerAction;

pub(crate) fn pause_plugin(app: &mut App) {
    app.add_system(toggle_input::<PlayerAction, false>.in_schedule(OnEnter(GameState::Paused)))
        .add_system(toggle_input::<PlayerAction, true>.in_schedule(OnExit(GameState::Paused)))
        .add_system(reset_state::<GameState>.run_if(state_changed::<AppState>()));
}

fn toggle_input<T: Actionlike, const ENABLE: bool>(
    mut cmd: Commands,
    toggle: Option<ResMut<ToggleActions<T>>>,
) {
    if let Some(mut toggle) = toggle {
        toggle.enabled = ENABLE;
    } else {
        let mut toggle = ToggleActions::<T>::default();
        toggle.enabled = ENABLE;
        cmd.insert_resource(toggle);
    }
}
