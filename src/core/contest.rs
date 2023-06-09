use crate::core::bridge::Bridge;
use crate::core::game_history::*;
use crate::core::player::*;
use crate::core::traits::*;
use crate::core::{BuilderError, Error};
use std::time::Instant;

/// **API note:** Agents are moved into the contest, as they should not be reused.
pub struct Contest<G, A, B, BrA, BrB>
where
    G: Game,
    A: Agent<Game = G>,
    B: Agent<Game = G>,
    BrA: Bridge<A>,
    BrB: Bridge<B>,
{
    state: G::State,
    history: GameHistory<G>,
    player_a: Player<A, BrA>,
    player_b: Player<B, BrB>,
    encountered_error: bool,
}

impl<G, A, B, BrA, BrB> Contest<G, A, B, BrA, BrB>
where
    G: Game,
    A: Agent<Game = G>,
    B: Agent<Game = G>,
    BrA: Bridge<A>,
    BrB: Bridge<B>,
{
    /// Returns the result of the game or None if the game is not over.
    pub fn game_result(&self) -> Option<G::GameResult> {
        self.state.game_result()
    }

    /// Returns a reference to Agent A.
    pub fn agent_a(&self) -> &Player<A, BrA> {
        &self.player_a
    }

    /// Returns a reference to Agent B.
    pub fn agent_b(&self) -> &Player<B, BrB> {
        &self.player_b
    }

    fn is_over(&self) -> bool {
        self.state.is_terminal()
    }

    /// Returns a reference to the current state of the game.
    pub fn state(&self) -> &G::State {
        &self.state
    }

    /// Plays out a full game between the two agents and returns the result.
    pub fn play(&mut self) -> Result<G::GameResult, Error> {
        for res in &mut *self {
            if res.is_err() {
                return Err(res.unwrap_err());
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

impl<G, A, B, BrA, BrB> Iterator for &mut Contest<G, A, B, BrA, BrB>
where
    G: Game,
    A: Agent<Game = G>,
    B: Agent<Game = G>,
    BrA: Bridge<A>,
    BrB: Bridge<B>,
{
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

        let agent_start = Instant::now();

        // ply 1 is the first action, so agent A starts
        let action = if self.state.ply() % 2 == 1 {
            self.player_a.recommend_action(&self.state)
        } else {
            self.player_b.recommend_action(&self.state)
        };
        let agent_time = agent_start.elapsed();

        // if action is error, we need to return the error and stop the contest
        if let Err(e) = action {
            self.encountered_error = true;
            return Some(Err(e));
        }

        // if the agent took too long, we need to return the error and stop the contest
        if self.state.ply() % 2 == 1 {
            if agent_time > self.agent_a().time_limit() {
                self.encountered_error = true;
                return Some(Err(Error::TimeLimitExceeded));
            }
        } else {
            if agent_time > self.agent_b().time_limit() {
                self.encountered_error = true;
                return Some(Err(Error::TimeLimitExceeded));
            }
        }

        let action = action.unwrap(); // unwrap checked above

        if !self.state.is_legal(&action) {
            self.encountered_error = true;
            return Some(Err(Error::IllegalAction));
        }

        // apply the action, finishing the turn - INVARIANT IS RESTORED
        self.state = self.state.apply_action(&action);

        // update the history
        self.history
            .add_turn(action.clone(), self.state.clone(), agent_time);

        Some(Ok((old_state, action, self.state.clone())))
    }
}

pub struct ContestBuilder<G, A, B, BrA, BrB>
where
    G: Game,
    A: Agent<Game = G>,
    B: Agent<Game = G>,
    BrA: Bridge<A>,
    BrB: Bridge<B>,
{
    state: Option<G::State>,
    player_a: Option<Player<A, BrA>>,
    player_b: Option<Player<B, BrB>>,
}

impl<G, A, B, BrA, BrB> ContestBuilder<G, A, B, BrA, BrB>
where
    G: Game,
    A: Agent<Game = G>,
    B: Agent<Game = G>,
    BrA: Bridge<A>,
    BrB: Bridge<B>,
{
    /// Create a new contest builder
    pub fn new() -> Self {
        Self {
            state: None,
            player_a: None,
            player_b: None,
        }
    }

    pub fn initial_state(mut self, state: G::State) -> Self {
        self.state = Some(state);
        self
    }

    /// Set the player that will start the game.
    pub fn player_starts(mut self, player: Player<A, BrA>) -> Self {
        self.player_a = Some(player);
        self
    }

    /// Set the player that will play second.
    pub fn plays_aginst(mut self, player: Player<B, BrB>) -> Self {
        self.player_b = Some(player);
        self
    }

    /// Build the contest, returning an error if any required attributes are missing.
    pub fn build(mut self) -> Result<Contest<G, A, B, BrA, BrB>, BuilderError> {
        if self.state.is_none() {
            self.state = Some(G::initial_state());
        }
        if self.player_a.is_none() {
            return Err(BuilderError::MissingAttribute("Player A not set"));
        }
        if self.player_b.is_none() {
            return Err(BuilderError::MissingAttribute("Player B not set"));
        }
        Ok(Contest {
            state: self.state.clone().unwrap(),
            history: GameHistory::new(
                self.player_a.as_ref().unwrap().name().clone().to_string(),
                self.player_b.as_ref().unwrap().name().clone().to_string(),
                self.state.clone().unwrap(),
            ),
            player_a: self.player_a.unwrap(),
            player_b: self.player_b.unwrap(),
            encountered_error: false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agents::simple_agent::SimpleAgent;
    use crate::games::counting_game::*;

    #[test]
    fn test_history() {
        let agent1: SimpleAgent<CountingGame> = SimpleAgent::default();
        let agent2: SimpleAgent<CountingGame> = SimpleAgent::default();

        let mut contest = ContestBuilder::new()
            .initial_state(CountingGame::initial_state())
            .player_starts(
                PlayerBuilder::new()
                    .name("Monke-1")
                    .agent(agent1)
                    .build()
                    .unwrap(),
            )
            .plays_aginst(
                PlayerBuilder::new()
                    .name("Monke-2")
                    .agent(agent2)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();
        contest.play();
        let history = contest.history();
        assert_ne!(history.turns().len(), 0);
    }
}
