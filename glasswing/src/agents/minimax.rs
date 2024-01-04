use crate::agents::{sort_actions, Evaluator};
use crate::core::{Game, GwState, GwTeam, Polarity};
use num_traits::Bounded;
use std::marker::PhantomData;

pub struct MiniMax<G, H> {
    depth: u32,
    heuristic: H,
    pub states_evaluated: u64,
    _game: PhantomData<G>,
}

impl<G, H> MiniMax<G, H>
where
    G: Game,
    G::EvalType: Ord + Bounded + Copy,
    H: Evaluator<G>,
{
    pub fn new(depth: u32, heuristic: H) -> Self {
        MiniMax {
            depth,
            heuristic,
            states_evaluated: 0,
            _game: PhantomData,
        }
    }

    pub fn minimax(
        &mut self,
        state: &<G as Game>::State,
        depth: u32,
        mut alpha: G::EvalType,
        mut beta: G::EvalType,
    ) -> G::EvalType {
        if depth == 0 || state.is_terminal() {
            self.states_evaluated += 1;
            return self.heuristic.evaluate(state);
        }

        let mut actions = state.actions().into_iter().collect::<Vec<G::Action>>();
        sort_actions(state, &mut actions, &mut self.heuristic);

        match state.team_to_move().polarity() {
            Polarity::Positive => {
                // maximizing
                let mut value = G::EvalType::min_value();
                for action in actions {
                    let new_state = state.apply_action(&action);
                    value = value.max(self.minimax(&new_state, depth - 1, alpha, beta));
                    alpha = alpha.max(value);
                    if alpha >= beta {
                        break; // Beta cut-off
                    }
                }
                value
            }
            Polarity::Negative => {
                // minimizing
                let mut value = G::EvalType::max_value();
                for action in actions {
                    let new_state = state.apply_action(&action);
                    value = value.min(self.minimax(&new_state, depth - 1, alpha, beta));
                    beta = beta.min(value);
                    if beta <= alpha {
                        break; // Alpha cut-off
                    }
                }
                value
            }
        }
    }
}

impl<G, E> Evaluator<G> for MiniMax<G, E>
where
    G: Game,
    G::EvalType: Ord + Bounded + Copy,
    E: Evaluator<G>,
{
    fn evaluate(&mut self, state: &G::State) -> G::EvalType {
        let val = self.minimax(
            state,
            self.depth,
            G::EvalType::min_value(),
            G::EvalType::max_value(),
        );
        val
    }
}
