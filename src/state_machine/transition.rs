use super::trigger::StateTrigger;

pub struct Transition<TState> {
    pub(super) trigger: StateTrigger,
    pub(super) to: TState,
}

impl<TState: Clone> Transition<TState> {
    pub fn new(trigger: StateTrigger, to: TState) -> Self {
        Self { trigger, to }
    }

    pub(super) fn from(mut self, from: TState) -> TransitionFrom<TState> {
        TransitionFrom::new(from, self.to.clone(), self.trigger.clone())
    }
}

impl<TState: Clone> From<(StateTrigger, TState)> for Transition<TState> {
    fn from((trigger, to): (StateTrigger, TState)) -> Self {
        Self::new(trigger, to)
    }
}

pub struct TransitionFrom<TState> {
    pub(super) from: TState,
    pub(super) to: TState,
    pub(super) trigger: StateTrigger,
}

impl<TState: Clone> TransitionFrom<TState> {
    pub fn new(from: TState, to: TState, trigger: StateTrigger) -> Self {
        Self { from, to, trigger }
    }

    pub(super) fn transition(mut self) -> Transition<TState> {
        Transition::new(self.trigger.clone(), self.to.clone())
    }
}

// pub struct TransitionBuilder<TState, TBuilder> {
//     transition: Option<TransitionBuilder<TState>>,
//     builder: TBuilder,
// }
