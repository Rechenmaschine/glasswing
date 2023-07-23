use crate::core::{Agent, Game, MatchError, State};
use anyhow::Error;
use log::debug;
use std::time::{Duration, Instant};

/// Any type that implements the [`Agent`] trait.
// 'static required by Match::new(), but not necessary for AnyAgent.
// Used only to give stronger guarantees.
type AnyAgent<G> = Box<dyn Agent<G> + 'static>;

// Utility implementation make this layer transparent to the user.
impl<G: Game> Agent<G> for AnyAgent<G> {
    fn recommend_action(
        &mut self,
        state: &G::State,
        time_limit: Duration,
    ) -> Result<G::Action, Error> {
        self.as_mut().recommend_action(state, time_limit)
    }
}

/// `Match` represents a match of type `G` between two agents playing a game.
///
/// The current state of the game is stored in `state`.
/// `agent1` and `agent2` represent the two agents participating in the game,
/// where `agent1` plays all even turns, and `agent2` plays all odd turns.
///
/// `Match` implements the `Iterator` trait. This allows the game to be
/// progressed in a step-by-step manner, producing the result of each turn
/// when iterated over. The item type of the iterator is a `Result` containing
/// the previous state, the performed action, and the resulting state, or an `Error`.
/// When the `next()` function is called, it advances the match to the next state,
/// calling the current player's agent to recommend an action and then applying the action
/// to the game state, producing the next state. If an error occurs during the
/// action selection or application, the error is returned and the match is terminated.
/// As such, any further calls to `next()` will return `None`.
///
/// A match can be configured to operate with various options:
///
/// * `check`: If this flag is true, each action recommended by an agent is
/// checked for its validity. If an action is invalid, the game is immediately
/// terminated with an [`InvalidAction`] error.
///
/// * `time_limit`: This sets the soft time limit for each turn. This is the target time
/// given to each agent for making their move. It does not enforce any hard limit.
///
/// * `enforce`: If this flag is true, then the `time_limit` becomes a hard limit. The game
/// will be terminated with a [`TimeLimitExceeded`] error if an agent exceeds the limit.
///
///
/// [`InvalidAction`]: ../enum.MatchError.html#variant.InvalidAction
/// [`TimeLimitExceeded`]: ../enum.MatchError.html#variant.TimeLimitExceeded
///
///
/// # Example
///
/// ```rust
/// # use std::time::Duration;
/// # use glasswing_core::prelude::*;
/// use game::YourGame;
///
/// let game = YourGame::new();
/// let agent1 = Agent1::new();
/// let agent2 = Agent2::new();
/// let mut contest = Match::new(agent1, agent2)
///     .with_init_state(game.initial_state())
///     .check_actions(true)
///     .with_time_limit(Duration::from_secs(5))
///     .enforce_time_limit(true);
///
/// // Step through the game turn by turn.
/// while let Some(result) = contest.next() {
///     match result {
///         Ok((prev_state, action, next_state)) => {
///             /*...*/
///         }
///         Err(e) => {
///             /*...*/
///         }
///     }
/// }
///
/// let result = contest.game_result().expect("Game should have ended");
/// ```
///
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
    /// Create a new match between two agents where `agent1` plays all even turns
    /// and `agent2` is plays all odd turns.
    ///
    /// - `state` is set to the initial state of the game per default. Use
    /// [`with_init_state`] to set a different initial state.
    ///
    /// - `time_limit` is set to `Duration::MAX` per default. Use [`with_time_limit`] to set
    /// a different time limit.
    ///
    /// - `check` is set to `false` per default. Use [`check_actions`] to change this.
    ///
    /// - `enforce` is set to `false` per default. Use [`enforce_time_limit`] to change this.
    ///
    /// Note that this method boxes the agents, such that they can be of different types.
    ///
    /// [`with_init_state`]: #method.with_init_state
    /// [`with_time_limit`]: #method.with_time_limit
    /// [`check_actions`]: #method.check_actions
    /// [`enforce_time_limit`]: #method.enforce_time_limit
    pub fn new(agent1: impl Agent<G> + 'static, agent2: impl Agent<G> + 'static) -> Self {
        let agent1: Box<dyn Agent<G> + 'static> = Box::new(agent1);
        let agent2: Box<dyn Agent<G> + 'static> = Box::new(agent2);
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

    /// Initializes the match with a given state.
    ///
    /// Note that if the state begins in a non-initial state, `agent1` may not necessarily
    /// be the first to move since `agent1` is mapped to all even turns and `agent2` is mapped
    /// to all odd turns.
    pub fn with_init_state(self, state: G::State) -> Self {
        Match { state, ..self }
    }

    /// Sets whether the each action should be checked for validity. If
    /// this option is set, the game checks that each action is valid.
    /// If the action is invalid, the game is immediately terminated
    /// with [`InvalidAction`].
    ///
    /// Any iteration after encountering an error will return `None`.
    ///
    /// [`InvalidAction`]: crate::core::MatchError#InvalidAction
    pub fn check_actions(self, check: bool) -> Self {
        Match { check, ..self }
    }

    /// Sets the soft time limit for each turn. This is the target time
    /// passed to each agent.
    ///
    /// Note that this is a soft limit, and the game will not terminate
    /// if the time limit is exceeded.
    pub fn with_time_limit(self, time_limit: Duration) -> Self {
        Match { time_limit, ..self }
    }

    /// Sets whether the time limit should be enforced. If this option
    /// is set, the game will terminate with [`TimeLimitExceeded`] if
    /// the time limit is exceeded.
    ///
    /// Any iteration after encountering an error will return `None`.
    ///
    /// [`TimeLimitExceeded`]: crate::core::MatchError#TimeLimitExceeded
    pub fn enforce_time_limit(self, enforce: bool) -> Self {
        Match { enforce, ..self }
    }

    /// Returns the current time limit
    pub fn time_limit(&self) -> Duration {
        self.time_limit
    }

    /// Returns the current state of the game
    pub fn state(&self) -> &G::State {
        &self.state
    }

    /// Returns the first agent, which plays all even turns
    pub fn agent1(&self) -> &(impl Agent<G> + 'static) {
        &self.agent1
    }

    /// Returns the second agent, which plays all odd turns
    pub fn agent2(&self) -> &(impl Agent<G> + 'static) {
        &self.agent2
    }

    /// Plays the game to completion, returning the game result or an error
    /// if the game terminated due to an error.
    pub fn playout(mut self) -> Result<Self, Error> {
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
        // if we encountered an error, the match may not continue
        if self.error {
            return None;
        }

        // if the game is over, the match may not continue
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

        // if the agent returned an error, the match terminates with an error
        if let Err(err) = action {
            self.error = true;
            return Some(Err(err));
        }

        // Check if the agent exceeded the time limit
        if self.enforce && agent_time > self.time_limit {
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
    use super::*;
    use crate::agents::minimax_agent::MiniMaxAgent;
    use crate::agents::random_agent::RandomAgent;
    use crate::core::TwoPlayerGameResult::Winner;
    use crate::core::TwoPlayerTeam::{One as X, Two as O};
    use crate::games::tic_tac_toe::{TicTacToe, TicTacToeEvaluator};
    use log::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;
    //use pretty_env_logger::env_logger::builder;

    #[test]
    fn test_match() {
        //builder().filter_level(LevelFilter::Trace).init();

        type TTT = TicTacToe<3>;

        let mut simple_won = 0;
        let mut rng_won = 0;

        for _ in 0..1 {
            let rng = RandomAgent::new(StdRng::from_entropy());
            let minmax = MiniMaxAgent::new(10, TicTacToeEvaluator);
            let match1 = Match::<TTT>::new(minmax, rng)
                .check_actions(true)
                .with_time_limit(Duration::ZERO)
                .enforce_time_limit(false);

            match match1.playout().unwrap().game_result().unwrap() {
                Winner(team) => match team {
                    X => {
                        simple_won += 1;
                    }
                    O => {
                        rng_won += 1;
                    }
                },
                _ => {}
            }
        }

        info!("Simple won {} times", simple_won);
        info!("RNG won {} times", rng_won);
    }
}
