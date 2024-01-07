use crate::core::Game;
use std::fmt;

pub trait GwState<G: Game<State = Self>>: Sized + Clone + fmt::Debug + Send + Sync {
    type ActionIter: IntoIterator<Item = G::Action>;

    fn actions(&self) -> Self::ActionIter;

    #[inline]
    fn substates(&self) -> SubStateIter<G> {
        SubStateIter::new(self.clone())
    }

    #[inline]
    fn count_actions(&self) -> usize {
        self.actions().into_iter().count()
    }

    fn team_to_move(&self) -> G::Team;

    #[must_use]
    fn apply_action(&self, action: &G::Action) -> Self;

    #[inline]
    fn is_terminal(&self) -> bool {
        self.game_result().is_some()
    }

    fn game_result(&self) -> Option<G::GameResult>;
}

pub struct SubStateIter<G: Game> {
    actions: <<<G as Game>::State as GwState<G>>::ActionIter as IntoIterator>::IntoIter,
    state: G::State,
}

impl<G: Game> SubStateIter<G> {
    #[inline]
    fn new(state: G::State) -> Self {
        Self {
            actions: state.actions().into_iter(),
            state,
        }
    }
}

impl<G: Game> Iterator for SubStateIter<G> {
    type Item = G::State;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.actions
            .next()
            .as_ref()
            .map(|action| self.state.apply_action(action))
    }
}
