use crate::agents::{sort_actions, Evaluator};
use crate::core::{Game, GwState};
use num_traits::Bounded;
use std::marker::PhantomData;
use std::ops::Neg;
use smallvec::SmallVec;

pub struct NegaMax<G, E>
where
    G: Game,
    G::EvalType: Ord + Bounded + Neg<Output = G::EvalType> + Copy,
    E: Evaluator<G>,
{
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
        // In most games we hit the depth limit before we hit a terminal state,
        // therefore it is more efficient to check for the depth limit first.
        if depth == 0 || state.is_terminal() {
            return self.evaluator.evaluate_for(state, &state.team_to_move());
        }

        // Generate all legal actions from the current state and sort in ascending order of heuristic.
        let mut actions = state.actions().into_iter().collect::<SmallVec<[G::Action; 8]>>();
        sort_actions(
            state,
            &mut actions,
            &mut self.evaluator,
            &state.team_to_move(),
        );

        // iterate in descending order as per negamax optimisation
        let mut value = -G::EvalType::max_value();
        for action in actions.iter().rev() {
            let new_state = state.apply_action(&action);
            let eval = -self.negamax(&new_state, depth - 1, -beta, -alpha);
            value = value.max(eval);
            alpha = alpha.max(value);
            if alpha >= beta {
                break; // (* cut-off *)
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
    fn evaluate_for(&mut self, state: &G::State, for_team: &G::Team) -> G::EvalType {
        if state.team_to_move() == *for_team {
            // Hacky workaround to avoid overflow. TODO fix properly.
            self.negamax(
                state,
                self.depth,
                -G::EvalType::max_value(),
                G::EvalType::max_value(),
            )
        } else {
            -self.negamax(
                state,
                self.depth,
                -G::EvalType::max_value(),
                G::EvalType::max_value(),
            )
        }
    }
}
