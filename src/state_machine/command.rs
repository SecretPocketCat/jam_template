use std::hash::Hash;

use bevy::prelude::*;

use super::trigger::Done;

pub trait CommandStateMachineExtensions {
    fn state_machine_done<TState>(&mut self, entity: Entity)
    where
        TState: Clone + Eq + PartialEq + Hash + Send + Sync + 'static;
}

impl CommandStateMachineExtensions for Commands<'_, '_> {
    fn state_machine_done<TState>(&mut self, entity: Entity)
    where
        TState: Clone + Eq + PartialEq + Hash + Send + Sync + 'static,
    {
        self.entity(entity).insert(Done::<TState>::default());
    }
}
