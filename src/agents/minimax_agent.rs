use crate::core::traits::*;
use anyhow::Error;
use std::marker::PhantomData;
use std::time::Duration;

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

    fn minimax(
        &self,
        state: &<G as Game>::State,
        depth: u32,
        alpha: f32,
        beta: f32,
        maximizing_player: bool,
    ) -> f32 {
        //TODO: return Result<f32, Error>
        if depth == 0 || state.is_terminal() {
            return self
                .evaluator
                .evaluate(&state.advance_ply())
                .expect("Evaluation failed");
        }

        let mut new_alpha = alpha;
        let mut new_beta = beta;

        if maximizing_player {
            let mut max_eval = f32::NEG_INFINITY;
            for action in state.actions() {
                let child = state.next_state(&action);
                max_eval = f32::max(
                    max_eval,
                    self.minimax(&child, depth - 1, new_alpha, new_beta, false),
                ) as f32;
                new_alpha = f32::max(new_alpha, max_eval) as f32;
                if new_beta <= new_alpha {
                    break;
                }
            }
            max_eval
        } else {
            let mut min_eval = f32::INFINITY;
            for action in state.actions() {
                let child = state.next_state(&action);
                min_eval = f32::min(
                    min_eval,
                    self.minimax(&child, depth - 1, new_alpha, new_beta, true),
                ) as f32;
                new_beta = f32::min(new_beta, min_eval) as f32;
                if new_beta <= new_alpha {
                    break;
                }
            }
            min_eval
        }
    }
}

impl<G: Game, E: Evaluator<G>> Agent<G> for MiniMaxAgent<G, E> {
    fn recommend_action(&mut self, state: &G::State, _: Duration) -> Result<G::Action, Error> {
        // By convention, the maximizing player is the starting team
        let maximizing_player = G::starting_team() == state.current_team();

        let mut best_eval = if maximizing_player {
            f32::NEG_INFINITY
        } else {
            f32::INFINITY
        };
        let mut best_action = None;

        for action in state.actions() {
            let child = state.next_state(&action);
            let eval = self.minimax(
                &child,
                self.depth - 1,
                f32::NEG_INFINITY,
                f32::INFINITY,
                !maximizing_player,
            );

            if (maximizing_player && eval > best_eval) || (!maximizing_player && eval < best_eval) {
                best_eval = eval;
                best_action = Some(action);
            }
        }

        best_action.ok_or(MatchError::<G>::NoAvailableActions(state.clone()).into())
    }
}
