use crate::core::{Evaluator, Game, Polarity, ProbabilisticState, State, Team};
use anyhow::Error;
use std::marker::PhantomData;
use std::time::Duration;

pub struct ExpectiMiniMaxAgent<G, E>
where
    G: Game,
    G::State: ProbabilisticState<G>,
    E: Evaluator<G>,
{
    depth: u32,
    evaluator: E,
    _game: PhantomData<G>,
}

impl<G, E> ExpectiMiniMaxAgent<G, E>
where
    G: Game,
    G::State: ProbabilisticState<G>,
    E: Evaluator<G>,
{
    pub fn new(depth: u32, evaluator: E) -> Self {
        ExpectiMiniMaxAgent {
            depth,
            evaluator,
            _game: PhantomData,
        }
    }

    pub fn expectiminimax(&mut self, state: &<G as Game>::State, depth: u32) -> Result<f32, Error> {
        if depth == 0 || state.is_terminal() {
            return self.evaluator.evaluate(state);
        }

        if state.is_random_state() {
            // return weighted average of all substates
            let substates = state
                .substates()
                .into_iter()
                .collect::<Vec<(G::State, f32)>>();

            let mut avg = 0.0;
            for (child, probability) in substates {
                let value = self.expectiminimax(&child, depth - 1)?;
                // weighted by probability of occurrence
                avg += value * probability;
            }
            Ok(avg)
        } else {
            // do normal minimax
            let actions = state.actions().into_iter().collect::<Vec<G::Action>>();

            match state.team_to_move().polarity() {
                Polarity::Positive => {
                    // maximizing
                    let mut value = f32::MIN;
                    for action in actions {
                        let new_state = state.apply_action(&action);
                        value = value.max(self.expectiminimax(&new_state, depth - 1)?);
                    }
                    Ok(value)
                }
                Polarity::Negative => {
                    // minimizing
                    let mut value = f32::MAX;
                    for action in actions {
                        let new_state = state.apply_action(&action);
                        value = value.min(self.expectiminimax(&new_state, depth - 1)?);
                    }
                    Ok(value)
                }
            }
        }
    }
}

impl<G, E> crate::core::Agent<G> for ExpectiMiniMaxAgent<G, E>
where
    G: Game,
    G::State: ProbabilisticState<G>,
    E: Evaluator<G>,
{
    fn select_action(
        &mut self,
        state: &<G as Game>::State,
        _: Duration,
    ) -> Result<G::Action, Error> {
        let actions = state.actions().into_iter().collect::<Vec<G::Action>>();

        let maximising = state.team_to_move().polarity() == Polarity::Positive;

        let mut best_action = None;
        let mut best_value = if maximising { f32::MIN } else { f32::MAX };

        for action in actions {
            let new_state = state.apply_action(&action);
            let value = self.expectiminimax(&new_state, self.depth - 1)?;

            if maximising {
                if value > best_value {
                    best_value = value;
                    best_action = Some(action);
                }
            } else {
                if value < best_value {
                    best_value = value;
                    best_action = Some(action);
                }
            }
        }

        best_action.ok_or_else(|| Error::msg("No action found"))
    }
}
