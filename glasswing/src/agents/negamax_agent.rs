use anyhow::Error;
use log::{debug, trace};
use std::time::Duration;

use crate::agents::sort_actions;
use crate::core::{Agent, Evaluator, Game, MatchError, State, Team};
use std::marker::PhantomData;

pub struct NegaMaxAgent<G: Game, E: Evaluator<G>> {
    depth: u32,
    evaluator: E,
    _game: PhantomData<G>,
}

impl<G: Game, E: Evaluator<G>> NegaMaxAgent<G, E> {
    pub fn new(depth: u32, evaluator: E) -> Self {
        NegaMaxAgent {
            depth,
            evaluator,
            _game: PhantomData,
        }
    }

    pub fn negamax(
        &mut self,
        state: &<G as Game>::State,
        depth: u32,
        mut alpha: f32,
        beta: f32,
    ) -> Result<f32, Error> {
        if depth == 0 || state.is_terminal() {
            return Ok(
                self.evaluator.evaluate(state)? * state.team_to_move().polarity().sign() as f32
            );
        }

        //sort moves by heuristic value
        let sorted_actions = sort_actions(state, state.actions(), &self.evaluator);

        let mut value = f32::MIN;
        for action in sorted_actions {
            let new_state = state.apply_action(&action);
            let score = -self.negamax(&new_state, depth - 1, -beta, -alpha)?;
            value = value.max(score);
            alpha = alpha.max(score);
            if alpha >= beta {
                break; // Beta cut-off
            }
        }
        Ok(value)
    }
}

impl<G: Game, E: Evaluator<G>> Agent<G> for NegaMaxAgent<G, E> {
    fn recommend_action(&mut self, state: &G::State, _: Duration) -> Result<G::Action, Error> {
        let mut best_action = None;
        let mut best_value = f32::MIN;
        let mut alpha = f32::MIN;
        let beta = f32::MAX;

        for action in state.actions() {
            let new_state = state.apply_action(&action);
            let value = -self.negamax(&new_state, self.depth - 1, -beta, -alpha)?;

            trace!("Considering action {:?} with value {}", action, value);

            if value > best_value {
                best_value = value;
                best_action = Some(action);
                alpha = alpha.max(best_value);
            }
        }

        debug!("Best action: {:?}, eval={}", best_action, best_value);

        best_action.ok_or(MatchError::<G>::NoAvailableActions(state.clone()).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agents::random_agent::RandomAgent;
    use crate::core::Match;
    use crate::core::TwoPlayerGameResult::{Draw, Winner};
    use crate::core::TwoPlayerTeam::{One as X, Two as O};
    use crate::games::tic_tac_toe::{TicTacToe, TicTacToeEvaluator};
    use log::info;
    use rand::rngs::OsRng;

    //use pretty_env_logger::env_logger::builder;

    #[test]
    fn test_alternating() {
        // init logger
        //builder().filter_level(log::LevelFilter::Debug).init();

        let mut wins_negamax = 0;
        let mut wins_random = 0;
        let mut draws = 0;

        for i in 0..100 {
            let minimax = NegaMaxAgent::new(10, TicTacToeEvaluator);
            let random = RandomAgent::new(OsRng::default());

            let match_: Match<TicTacToe<3>> = if i % 2 == 0 {
                Match::new(minimax, random)
            } else {
                Match::new(random, minimax)
            };

            match match_.playout() {
                Ok(result) => match result.game_result().expect("Game result should be present") {
                    Winner(winner) => {
                        if winner == X && i % 2 == 0 || winner == O && i % 2 == 1 {
                            wins_negamax += 1;
                            info!("Minimax won as team {:?}", winner)
                        } else if winner == O && i % 2 == 0 || winner == X && i % 2 == 1 {
                            wins_random += 1;
                            info!("Random won as team {:?}", winner)
                        } else {
                            unreachable!("Invalid state")
                        }
                    }
                    Draw => {
                        draws += 1;
                        info!("Draw")
                    }
                },
                Err(e) => {
                    panic!("Error: {}", e);
                }
            }

            if i % 10 == 9 {
                info!("\n======= STATISTICS =======\nWins minimax: {}\nWins random: {}\nDraws: {}\n==========================", wins_negamax, wins_random, draws);
            }
        }
        assert_eq!(wins_negamax + wins_random + draws, 100);
        assert_eq!(wins_random, 0); // negamax should always win
    }

    #[test]
    fn test_first_player() {
        // init logger
        //builder().filter_level(log::LevelFilter::Info).init();

        let mut wins_negamax = 0;
        let mut wins_random = 0;
        let mut draws = 0;

        for i in 0..100 {
            let minimax = NegaMaxAgent::new(10, TicTacToeEvaluator);
            let random = RandomAgent::new(OsRng::default());

            let match_: Match<TicTacToe<3>> = Match::new(minimax, random);

            match match_.playout() {
                Ok(result) => match result.game_result().expect("Game result should be present") {
                    Winner(winner) => match winner {
                        X => {
                            wins_negamax += 1;
                            info!("minimax won as team {:?}", winner);
                        }
                        O => {
                            wins_random += 1;
                            info!("random won as team {:?}", winner);
                        }
                    },
                    Draw => {
                        draws += 1;
                        info!("Draw")
                    }
                },
                Err(e) => {
                    panic!("Error: {}", e);
                }
            }

            if i % 10 == 9 {
                info!("\n======= STATISTICS =======\nWins minimax: {}\nWins random: {}\nDraws: {}\n==========================", wins_negamax, wins_random, draws);
            }
        }
        assert_eq!(wins_negamax + wins_random + draws, 100);
        assert_eq!(wins_random, 0); // negamax should always win
    }
}
