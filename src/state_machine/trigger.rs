use crate::tools::enum_tools::enum_variant_eq;
use bevy::{prelude::*, utils::HashSet};
use std::hash::Hash;
use std::{marker::PhantomData, time::Duration};

// todo: trigger systems

#[derive(Debug, Clone, Eq)]
pub enum StateTrigger {
    And(Box<(StateTrigger, StateTrigger)>),
    Or(Box<(StateTrigger, StateTrigger)>),
    Not(Box<StateTrigger>),
    Timer(Duration),
    TweenDone,
    DoneInserted,
    /* Single frame trigger */
    SingleFrame,
}

impl StateTrigger {
    pub fn and(self, other: StateTrigger) -> Self {
        Self::And(Box::new((self, other)))
    }

    pub fn or(self, other: StateTrigger) -> Self {
        Self::Or(Box::new((self, other)))
    }
}

impl Hash for StateTrigger {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
    }
}

impl PartialEq for StateTrigger {
    fn eq(&self, other: &Self) -> bool {
        enum_variant_eq(self, other)
    }
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Done<TState>(PhantomData<TState>);
impl<TState> Default for Done<TState> {
    fn default() -> Self {
        Self(Default::default())
    }
}

pub(super) fn was_triggered(trigger: &StateTrigger, triggered: &HashSet<StateTrigger>) -> bool {
    match trigger {
        StateTrigger::And(and) => {
            was_triggered(&and.0, triggered) && was_triggered(&and.1, triggered)
        }
        StateTrigger::Or(or) => was_triggered(&or.0, triggered) || was_triggered(&or.1, triggered),
        StateTrigger::Not(not) => !was_triggered(&not, triggered),
        t => triggered.get(t).is_some(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::utils::HashSet;
    use std::time::Duration;
    use test_case::test_case;

    #[test_case(StateTrigger::SingleFrame, vec![StateTrigger::SingleFrame] => true ; "SingleFrame trigger should match when triggered by itself")]
    #[test_case(StateTrigger::SingleFrame, vec![StateTrigger::TweenDone] => false ; "SingleFrame trigger should not match when other trigger is present")]
    #[test_case(
        StateTrigger::And(Box::new((
            StateTrigger::And(Box::new((
                StateTrigger::DoneInserted,
                StateTrigger::TweenDone)
            )),
            StateTrigger::Not(Box::new(StateTrigger::SingleFrame))
        ))),
        vec![
            StateTrigger::SingleFrame,
            StateTrigger::DoneInserted,
            StateTrigger::TweenDone
        ] => false ; "And trigger should not match when one of its triggers is absent"
    )]
    #[test_case(
        StateTrigger::And(Box::new((
            StateTrigger::SingleFrame,
            StateTrigger::And(Box::new((
                StateTrigger::DoneInserted,
                StateTrigger::TweenDone)
            )))
        )),
        vec![
            StateTrigger::SingleFrame,
            StateTrigger::DoneInserted,
            StateTrigger::TweenDone
        ] => true ; "And trigger should match when all of its triggers are present"
    )]
    #[test_case(
        StateTrigger::Or(Box::new((
            StateTrigger::TweenDone,
            StateTrigger::DoneInserted
        ))),
        vec![StateTrigger::SingleFrame] => false ; "Or trigger should not match when none of its triggers are present"
    )]
    #[test_case(
        StateTrigger::Or(Box::new((
            StateTrigger::TweenDone,
            StateTrigger::DoneInserted
        ))),
        vec![StateTrigger::TweenDone,
            StateTrigger::SingleFrame
        ] => true ; "Or trigger should match when some of its triggers are present"
    )]
    #[test_case(
        StateTrigger::Not(Box::new(StateTrigger::Timer(Duration::from_secs(5)))),
        vec![
            StateTrigger::SingleFrame,
            StateTrigger::DoneInserted
        ] => true ; "Not trigger should match when its trigger is absent"
    )]
    #[test_case(
        StateTrigger::Timer(Duration::from_secs(5)),
        vec![
            StateTrigger::Timer(Duration::from_secs(10)),
        ] => true ; "Timer trigger should match despite different duration"
    )]
    fn test_was_triggered(trigger: StateTrigger, triggered: Vec<StateTrigger>) -> bool {
        let triggered_set: HashSet<_> = triggered.into_iter().collect();
        was_triggered(&trigger, &triggered_set)
    }
}
