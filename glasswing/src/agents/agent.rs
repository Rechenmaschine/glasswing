use crate::agents::Evaluator;
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
    G::EvalType: Ord + Copy + std::fmt::Debug,
{
    fn select_action(&mut self, state: &G::State) -> Result<G::Action, Error> {
        let actions = state.actions().into_iter().collect::<Vec<G::Action>>();

        let best = actions.iter().map(|x| {
            let evaluation = self.evaluator.evaluate_action_for(state, x, &state.team_to_move());
            println!("Considering Action {:?} with eval: {:?}", x, evaluation);
            (x, evaluation)
        })
            .max_by_key(|(_, evaluation)| *evaluation);

        if let Some((action, eval)) = best {
            println!("Selected {:?} / eval: {:?}", action, eval);
            Ok(action.clone())
        } else {
            Err(MatchError::<G>::NoAvailableActions(state.clone()).into())
        }
    }
}
