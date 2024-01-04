use crate::agents::{Evaluator, sort_actions};
use crate::core::{Game, GwState, MatchError};
use anyhow::Error;
use std::marker::PhantomData;

pub trait Agent<G: Game> {
    fn select_action(&mut self, state: &G::State) -> Result<G::Action, Error>;
}

/// An agent which selects the best action for the current player according
/// to an evaluator.
pub struct MaximisingAgent<G: Game, E: Evaluator<G>> {
    evaluator: E,
    _marker: PhantomData<G>,
}

impl<G: Game, E: Evaluator<G>> MaximisingAgent<G, E> {
    pub fn new(evaluator: E) -> Self {
        MaximisingAgent {
            evaluator,
            _marker: PhantomData,
        }
    }

    pub fn evaluator(&self) -> &E {
        &self.evaluator
    }

    pub fn evaluator_mut(&mut self) -> &mut E {
        &mut self.evaluator
    }
}

impl<G, E> Agent<G> for MaximisingAgent<G, E>
where
    G: Game,
    E: Evaluator<G>,
    G::EvalType: Ord + Copy,
{
    fn select_action(&mut self, state: &G::State) -> Result<G::Action, Error> {
        let mut actions = state.actions().into_iter().collect::<Vec<G::Action>>();
        sort_actions(state, &mut actions, &mut self.evaluator, &state.team_to_move());

        actions.last()
            .ok_or_else(|| MatchError::<G>::NoAvailableActions(state.clone()).into())
            .cloned()
    }
}
