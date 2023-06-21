use crate::core::game_history::*;
use crate::core::traits::*;
use anyhow::Error;
use std::time::{Duration, Instant};
use thiserror::Error;

#[derive(Error, Debug)]
enum ContestError<G: Game> {
    #[error("Player exceeded time limit (Agent took {time:?}, exceeding {limit:?})")]
    TimeLimitExceeded { time: Duration, limit: Duration },
    #[error("Invalid action in state (provided {action:?} in {state:?}")]
    InvalidAction { action: G::Action, state: G::State },
}

type AnyAgent<G> = Box<dyn Agent<G> + Send>;

impl<G: Game> Agent<G> for Box<dyn Agent<G> + Send> {
    fn recommend_action(
        &mut self,
        state: &G::State,
        time_limit: Duration,
    ) -> Result<G::Action, Error> {
        self.as_mut().recommend_action(state, time_limit)
    }
}

pub trait IntoAgent<G: Game> {
    fn boxed(self) -> AnyAgent<G>;
}

impl<A: Agent<G> + 'static, G: Game> IntoAgent<G> for A {
    fn boxed(self) -> AnyAgent<G> {
        let agent: Box<dyn Agent<G> + Send> = Box::new(self);
        agent
    }
}

pub struct Player<G: Game> {
    agent: AnyAgent<G>,
    name: &'static str,
    time_limit: Duration,
}

impl<G: Game> Player<G> {
    pub fn new(agent: impl Agent<G> + 'static) -> Self {
        let name = std::any::type_name_of_val(&agent);
        let agent: Box<dyn Agent<G> + 'static + Send> = Box::new(agent);
        Player {
            agent,
            name,
            time_limit: Duration::MAX,
        }
    }

    pub fn with_name(self, name: &'static str) -> Self {
        Player { name, ..self }
    }

    pub fn with_time_limit(self, time_limit: Duration) -> Self {
        Player { time_limit, ..self }
    }

    pub fn get_action(&mut self, state: &G::State) -> Result<G::Action, Error> {
        self.agent.recommend_action(state, self.time_limit)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn time_limit(&self) -> Duration {
        self.time_limit
    }
}

/// **API note:** Agents are moved into the contest, as they should not be reused.
pub struct Contest<G: Game> {
    state: G::State,
    history: GameHistory<G>,
    player1: Player<G>,
    player2: Player<G>,
    encountered_error: bool,
}

impl<G: Game> Contest<G> {
    pub fn new(player1: Player<G>, player2: Player<G>) -> Self {
        Contest {
            state: G::initial_state(),
            history: GameHistory::new(
                player1.name.to_string(),
                player2.name.to_string(),
                G::initial_state(),
            ),
            player1,
            player2,
            encountered_error: false,
        }
    }

    pub fn with_initial_state(self) -> Self {
        Contest {
            state: G::initial_state(),
            history: GameHistory::new(
                self.player1.name.to_string(),
                self.player2.name.to_string(),
                self.state,
            ),
            ..self
        }
    }

    /// Returns the result of the game or None if the game is not over.
    pub fn game_result(&self) -> Option<G::GameResult> {
        self.state.game_result()
    }

    /// Returns a reference to Agent A.
    pub fn player_1(&self) -> &Player<G> {
        &self.player1
    }

    /// Returns a reference to Agent B.
    pub fn player_2(&self) -> &Player<G> {
        &self.player2
    }

    /// Returns true if the game is over, or if an error has been encountered.
    pub fn is_over(&self) -> bool {
        self.state.is_terminal() || self.encountered_error
    }

    /// Returns a reference to the current state of the game.
    pub fn state(&self) -> &G::State {
        &self.state
    }

    /// Plays out a full game between the two agents and returns the result.
    pub fn play(&mut self) -> Result<G::GameResult, Error> {
        for res in &mut *self {
            if res.is_err() {
                return Err(res.err().unwrap());
            }
        }
        return Ok(self
            .state
            .game_result()
            .expect("No game result provided even though game is over"));
    }

    pub fn history(&self) -> &GameHistory<G> {
        &self.history
    }
}

impl<G: Game> Iterator for &mut Contest<G> {
    type Item = Result<(G::State, G::Action, G::State), Error>;

    /// Advances the ManagedContest to the next state, calling each agent in turn
    /// to recommend an action and then yielding the action and resulting state.
    ///
    /// ## Usage:
    /// ```
    /// for (old_state, action, cur_state) in &mut ManagedContest {
    ///     ...
    /// }
    /// ```
    ///
    /// For access to the ManagedContest, use the following syntax instead:
    /// ```
    /// while let Some((old_state, action, cur_state)) = (&mut ManagedContest).next() {
    ///     ...
    /// }
    /// ```
    fn next(&mut self) -> Option<Self::Item> {
        if self.encountered_error {
            return None;
        }

        if self.state.is_terminal() {
            return None;
        }

        let old_state = self.state.clone();

        // increment the turn - INVARIANT IS BROKEN
        self.state = self.state.advance_ply();

        // ply 1 is the first action, so agent A starts
        let player = if self.state.ply() % 2 == 1 {
            &mut self.player1
        } else {
            &mut self.player2
        };

        let agent_start = Instant::now();
        let action = player.get_action(&self.state);
        let agent_time = agent_start.elapsed();

        // if agent encountered an error, we need to return the error and stop the contest
        if let Err(e) = action {
            self.encountered_error = true;
            return Some(Err(e));
        }

        // if the agent took too long, we need to return the error and stop the contest
        if agent_time > player.time_limit() {
            self.encountered_error = true;
            return Some(Err(ContestError::<G>::TimeLimitExceeded {
                time: agent_time,
                limit: player.time_limit(),
            }
            .into()));
        }

        let action = action.unwrap(); // unwrap checked above

        if !self.state.is_legal(&action) {
            self.encountered_error = true;
            return Some(Err(ContestError::<G>::InvalidAction {
                action: action.clone(),
                state: self.state.clone(),
            }
            .into()));
        }

        // apply the action, finishing the turn - INVARIANT IS RESTORED
        self.state = self.state.apply_action(&action);

        // update the history
        self.history
            .add_turn(action.clone(), self.state.clone(), agent_time);

        Some(Ok((old_state, action, self.state.clone())))
    }
}

#[cfg(feature = "tournaments")]
use tournament_rs::prelude::{Match, Player as TournamentPlayer};

#[cfg(feature = "tournaments")]
impl<G: Game> Match for Contest<G> {
    type Agent = Player<G>;
    type MatchResult = G::GameResult;

    fn new(player1: TournamentPlayer<Self>, player2: TournamentPlayer<Self>) -> Self {
        let p1 = player1.unpack();
        let p2 = player2.unpack();

        Contest::new(p1, p2)
    }

    fn playout(&mut self) -> Result<Self::MatchResult, Error> {
        return self.play();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agents::minimax_agent::MiniMaxAgent;
    use crate::agents::simple_agent::SimpleAgent;
    use crate::games::counting_game::{CountingGame, CountingGameEvaluator};

    #[test]
    fn test_contest() {
        let mut contest = Contest::<CountingGame>::new(
            Player::new(SimpleAgent::new()).with_name("Simple"),
            Player::new(MiniMaxAgent::new(10, CountingGameEvaluator)).with_name("MiniMax"),
        );

        let result = contest.play().unwrap();
        println!("Result: {:?}", result);
    }
}
