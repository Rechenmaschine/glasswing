use crate::core::Game;
use std::fmt::Debug;

pub trait GwState<G>
where
    Self: Sized + Clone + Debug + Send + Sync,
    G: Game<State = Self>,
{
    type ActionIter: IntoIterator<Item = G::Action>;

    #[inline]
    fn is_legal(&self, action: &G::Action) -> bool {
        self.actions().into_iter().any(|a| a == *action)
    }

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

    fn is_terminal(&self) -> bool;

    fn game_result(&self) -> Option<G::GameResult>;
}

pub struct SubStateIter<G>
where
    G: Game,
{
    actions: <<<G as Game>::State as GwState<G>>::ActionIter as IntoIterator>::IntoIter,
    state: G::State,
}

impl<G> SubStateIter<G>
where
    G: Game,
{
    #[inline]
    fn new(state: G::State) -> Self {
        Self {
            actions: state.actions().into_iter(),
            state,
        }
    }
}

impl<G> Iterator for SubStateIter<G>
where
    G: Game,
{
    type Item = G::State;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.actions
            .next()
            .as_ref()
            .map(|action| self.state.apply_action(action))
    }
}
