use crate::agents::{sort_actions, Evaluator};
use crate::core::{Game, GwState, GwTeam, Polarity};
use num_traits::Bounded;
use std::marker::PhantomData;
use std::ops::Neg;

pub struct NegaMax<G, E> {
    depth: u32,
    evaluator: E,
    _game: PhantomData<G>,
}

impl<G, E> NegaMax<G, E>
where
    G: Game,
    G::EvalType: Ord + Bounded + Neg<Output = G::EvalType> + Copy,
    E: Evaluator<G>,
{
    pub fn new(depth: u32, evaluator: E) -> Self {
        NegaMax {
            depth,
            evaluator,
            _game: PhantomData,
        }
    }

    pub fn negamax(
        &mut self,
        state: &G::State,
        depth: u32,
        mut alpha: G::EvalType,
        beta: G::EvalType,
    ) -> G::EvalType {
        if depth == 0 || state.is_terminal() {
            return match state.team_to_move().polarity() {
                Polarity::Positive => self.evaluator.evaluate(state),
                Polarity::Negative => -self.evaluator.evaluate(state),
            };
        }
        //sort moves by heuristic value
        let mut sorted_actions = state.actions().into_iter().collect::<Vec<G::Action>>();
        sort_actions(state, &mut sorted_actions, &mut self.evaluator);

        let mut value = G::EvalType::min_value();
        for action in sorted_actions {
            let new_state = state.apply_action(&action);
            let score = -self.negamax(&new_state, depth - 1, -beta, -alpha);
            value = value.max(score);
            alpha = alpha.max(score);
            if alpha >= beta {
                break; // Beta cut-off
            }
        }
        value
    }
}

impl<G, E> Evaluator<G> for NegaMax<G, E>
where
    G: Game,
    G::EvalType: Ord + Bounded + Neg<Output = G::EvalType> + Copy,
    E: Evaluator<G>,
{
    fn evaluate(&mut self, state: &G::State) -> G::EvalType {
        let val = self.negamax(
            state,
            self.depth,
            G::EvalType::min_value(),
            G::EvalType::max_value(),
        );
        val
    }
}
