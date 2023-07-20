use anyhow::Error;
use log::{debug, trace};
use std::time::Duration;

use crate::core::{Agent, Evaluator, Game, MatchError, State};
use std::marker::PhantomData;

pub struct MiniMaxAgent<G: Game, E: Evaluator<G>> {
    depth: u32,
    evaluator: E,
    _game: PhantomData<G>,
}

impl<G: Game, E: Evaluator<G>> MiniMaxAgent<G, E> {
    pub fn new(depth: u32, evaluator: E) -> Self {
        MiniMaxAgent {
            depth,
            evaluator,
            _game: PhantomData,
        }
    }

    pub fn minimax(
        &mut self,
        state: &<G as Game>::State,
        depth: u32,
        mut alpha: f32,
        mut beta: f32,
    ) -> f32 {
        if depth == 0 || state.is_terminal() {
            return self.evaluator.evaluate(state).unwrap();
        }

        let maximizing_player = G::starting_team() == state.team_to_move();

        if maximizing_player {
            let mut value = f32::MIN;
            for action in state.actions() {
                let new_state = state.apply_action(&action);
                value = value.max(self.minimax(&new_state, depth - 1, alpha, beta));
                alpha = alpha.max(value);
                if alpha >= beta {
                    break; // Beta cut-off
                }
            }
            value
        } else {
            let mut value = f32::MAX;
            for action in state.actions() {
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

impl<G: Game, E: Evaluator<G>> Agent<G> for MiniMaxAgent<G, E> {
    fn recommend_action(&mut self, state: &G::State, _: Duration) -> Result<G::Action, Error> {
        let maximizing_player = G::starting_team() == state.team_to_move();
        let mut best_action = None;
        let mut best_value = if maximizing_player {
            f32::MIN
        } else {
            f32::MAX
        };
        let mut alpha = f32::MIN;
        let mut beta = f32::MAX;

        for action in state.actions() {
            let new_state = state.apply_action(&action);
            let value = self.minimax(&new_state, self.depth - 1, alpha, beta);

            trace!("Considering action {:?} with value {}", action, value);

            if (maximizing_player && value > best_value)
                || (!maximizing_player && value < best_value)
            {
                best_value = value;
                best_action = Some(action);
                if maximizing_player {
                    alpha = alpha.max(best_value);
                } else {
                    beta = beta.min(best_value);
                }
            }
        }

        debug!("Best action: {:?}, eval={}", best_action, best_value);

        best_action.ok_or(MatchError::<G>::NoAvailableActions(state.clone()).into())
    }
}
