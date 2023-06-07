use crate::core::traits::*;
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

    fn minimax(
        &self,
        state: &<G as Game>::State,
        depth: u32,
        alpha: f32,
        beta: f32,
        maximizing_player: bool,
    ) -> f32 {
        if depth == 0 || state.is_terminal() {
            return self.evaluator.evaluate(state);
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

impl<G: Game, E: Evaluator<G>> Agent for MiniMaxAgent<G, E> {
    type Game = G;

    fn recommend_move(
        &mut self,
        state: &<<Self as Agent>::Game as Game>::State,
    ) -> <G as Game>::Action {
        let mut best_eval = f32::NEG_INFINITY;
        let mut best_action = None;

        // By convention, the maximizing player is the starting team
        let maximizing_player = G::starting_team() == state.current_team(); // TODO: Is this sensible?

        for action in state.actions() {
            let child = state.next_state(&action);
            let eval = self.minimax(
                &child,
                self.depth - 1,
                f32::NEG_INFINITY,
                f32::INFINITY,
                maximizing_player,
            );
            if eval > best_eval {
                best_eval = eval;
                best_action = Some(action);
            }
        }

        best_action.expect("No actions available for state")
    }
}
