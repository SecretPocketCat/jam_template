use super::{
    transition::{Transition, TransitionFrom},
    trigger::{was_triggered, StateTrigger},
};
use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};
use std::hash::Hash;

#[derive(Component)]
pub struct StateMachine<TState: Clone + Eq + PartialEq + Hash + Send + Sync + 'static> {
    state: TState,
    transitions_from: HashMap<TState, Vec<Transition<TState>>>,
    triggered: HashSet<StateTrigger>,
}

impl<TState: Clone + Eq + PartialEq + Hash + Send + Sync + 'static> StateMachine<TState> {
    pub fn new(
        initial_state: TState,
        transitions: impl IntoIterator<Item = TransitionFrom<TState>>,
    ) -> Self {
        let mut sm = Self {
            state: initial_state,
            transitions_from: default(),
            triggered: default(),
        };

        for trans in transitions.into_iter() {
            sm.transitions_from
                .entry(trans.from.clone())
                .or_insert(vec![])
                .push(trans.transition());
        }

        sm
    }

    pub fn state(&self) -> &TState {
        &self.state
    }

    fn set_state(&mut self, state: TState) {
        self.state = state;
    }
}

fn update_states<TState: Clone + Eq + PartialEq + Hash + Send + Sync + 'static>(
    mut sm_q: Query<(Entity, &mut StateMachine<TState>)>,
) {
    for (sm_e, mut sm) in sm_q.iter_mut() {
        if let Some(to) = sm.transitions_from.get(&sm.state).and_then(|transitions| {
            transitions
                .iter()
                .find(|trans| was_triggered(&trans.trigger, &sm.triggered))
                .map(|trans| trans.to.clone())
        }) {
            sm.set_state(to);
        }
    }
}
