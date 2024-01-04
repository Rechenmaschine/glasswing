use crate::agents::Evaluator;
use crate::core::{Game, GwState, GwTeam, MatchError, Polarity};
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
        let action_scores = state.actions().into_iter().map(|action| {
            let new_state = state.apply_action(&action);
            let score = self.evaluator.evaluate(&new_state);
            (action, score)
        });

        let best = match state.team_to_move().polarity() {
            Polarity::Positive => action_scores.max_by_key(|(_, score)| *score),
            Polarity::Negative => action_scores.min_by_key(|(_, score)| *score),
        };

        best.ok_or_else(|| MatchError::<G>::NoAvailableActions(state.clone()).into())
            .map(|(action, _)| action)
    }
}
