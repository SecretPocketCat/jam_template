use bevy::prelude::*;
use std::{hash::Hash, time::Duration};

use super::{
    state_machine::StateMachine,
    transition::{Transition, TransitionFrom},
    trigger::StateTrigger,
};

impl<TState: Clone + Eq + PartialEq + Hash + Send + Sync + 'static> StateMachine<TState> {
    pub fn new_sequence<TTransition: Into<Transition<TState>>>(
        initial_state: TState,
        transition_sequence: impl IntoIterator<Item = TTransition>,
    ) -> Self {
        let mut current_state = initial_state.clone();

        Self::new(
            initial_state,
            transition_sequence
                .into_iter()
                .map(|t| {
                    let transition = t.into().from(current_state.clone());
                    current_state = transition.to.clone();
                    transition
                })
                .collect::<Vec<_>>(),
        )
    }
}
