use crate::core::{Agent, Game, MatchError, State};
use anyhow::Error;
use log::debug;
use std::time::{Duration, Instant};

/// Any agent that implements the Agent trait.
type AnyAgent<G> = Box<dyn Agent<G> + 'static + Send>;

impl<G: Game> Agent<G> for AnyAgent<G> {
    fn recommend_action(
        &mut self,
        state: &G::State,
        time_limit: Duration,
    ) -> Result<G::Action, Error> {
        self.as_mut().recommend_action(state, time_limit)
    }
}

pub struct Match<G: Game> {
    agent1: AnyAgent<G>,
    agent2: AnyAgent<G>,
    state: G::State,
    error: bool,
    check: bool,
    time_limit: Duration,
    enforce: bool,
}

impl<G: Game> Match<G> {
    /// Create a new match between two agents.
    /// Agent1 starts the match.
    pub fn new(agent1: impl Agent<G> + 'static, agent2: impl Agent<G> + 'static) -> Self {
        let agent1: Box<dyn Agent<G> + 'static + Send> = Box::new(agent1);
        let agent2: Box<dyn Agent<G> + 'static + Send> = Box::new(agent2);
        Match {
            agent1,
            agent2,
            state: G::initial_state(),
            error: false,
            check: false,
            time_limit: Duration::MAX,
            enforce: false,
        }
    }

    /// Sets the initial state of the game
    pub fn with_init_state(self, state: G::State) -> Self {
        Match { state, ..self }
    }

    /// Sets whether the each action should be checked for validity
    pub fn check_actions(self, check: bool) -> Self {
        Match { check, ..self }
    }

    pub fn with_time_limit(self, time_limit: Duration) -> Self {
        Match { time_limit, ..self }
    }

    pub fn enforce_time_limit(self, enforce: bool) -> Self {
        Match { enforce, ..self }
    }

    pub fn time_limit(&self) -> Duration {
        self.time_limit
    }

    /// Returns the current state of the game
    pub fn state(&self) -> &G::State {
        &self.state
    }

    /// Returns the first agent
    pub fn agent1(&self) -> &(impl Agent<G> + 'static) {
        &self.agent1
    }

    /// Returns the second agent
    pub fn agent2(&self) -> &(impl Agent<G> + 'static) {
        &self.agent2
    }

    /// Plays the game to completion, returning the game result or an error
    /// if the game terminated prematurely
    pub fn playout(mut self) -> Result<Self, Error> {
        // while the game is not over or we encounter an error, keep playing
        while let Some(x) = self.next() {
            match x {
                Ok(_) => (),
                Err(err) => {
                    return Err(err);
                }
            }
        }
        Ok(self)
    }

    /// Returns the result of the game or None if the game is not over
    pub fn game_result(&self) -> Option<G::GameResult> {
        self.state.game_result()
    }
}

impl<G: Game> Iterator for Match<G> {
    type Item = Result<(G::State, G::Action, G::State), Error>;

    /// Advances the match to the next state, calling each agent in turn
    /// to recommend an action and then yielding the action and resulting state.
    fn next(&mut self) -> Option<Self::Item> {
        if self.error {
            return None;
        }

        if self.state.is_terminal() {
            return None;
        }

        let old_state = self.state.clone();

        // ply 1 is the first action, so agent A starts
        let player = if self.state.turn() % 2 == 0 {
            &mut self.agent1
        } else {
            &mut self.agent2
        };

        let start = Instant::now();
        let action = player.recommend_action(&self.state, self.time_limit);
        let agent_time = start.elapsed();

        // if agent encountered an error, we need to return the error and stop the contest
        if let Err(err) = action {
            self.error = true;
            return Some(Err(err));
        }

        // Check time limit but only throw error if "enforced"
        if agent_time > self.time_limit && self.enforce {
            self.error = true;
            return Some(Err(MatchError::<G>::TimeLimitExceeded {
                limit: self.time_limit,
                time: agent_time,
            }
            .into()));
        }

        let action = action.unwrap(); // unwrap checked above

        if self.check {
            // check that the action is valid
            if !self.state.is_legal(&action) {
                self.error = true;
                return Some(Err(MatchError::<G>::IllegalAction {
                    action,
                    state: self.state.clone(),
                }
                .into()));
            }
        }

        // apply the action to the state, finishing the turn
        self.state = self.state.apply_action(&action);

        debug!("Applied action: {:?}\n{:?}", action, self.state);

        Some(Ok((old_state, action, self.state.clone())))
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use crate::agents::minimax_agent::MiniMaxAgent;
    use crate::agents::random_agent::RandomAgent;
    use crate::agents::simple_agent::SimpleAgent;
    use crate::games::counting_game::CountingGameResult::Winner;
    use crate::games::counting_game::CountingTeam::*;
    use crate::games::counting_game::{CountingGame, CountingGameEvaluator};
    use log::*;
    use pretty_env_logger::env_logger::builder;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_match() {
        builder().filter_level(LevelFilter::Trace).init();

        let mut simple_won = 0;
        let mut rng_won = 0;

        for _ in 0..1 {
            let rng = RandomAgent::new(StdRng::from_entropy());
            let minmax = MiniMaxAgent::new(10, CountingGameEvaluator);
            let match1 = Match::<CountingGame>::new(minmax, rng)
                .check_actions(true)
                .with_time_limit(Duration::ZERO)
                .enforce_time_limit(false);

            match match1.playout().unwrap().game_result().unwrap() {
                Winner(team) => match team {
                    One => simple_won += 1,
                    Two => rng_won += 1,
                },
                _ => {}
            }
        }

        info!("Simple won {} times", simple_won);
        info!("RNG won {} times", rng_won);
    }
}
