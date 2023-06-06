use crate::core::move_history::GameHistory;
use crate::core::traits::*;

/// A match between two agents in a game.
pub struct ManagedContest<'a, G: Game, A: Agent<Game=G>, B: Agent<Game=G>> {
    state: G::State,
    history: GameHistory<G>,
    agent_a: &'a mut A,
    agent_b: &'a mut B,
}

impl<'a, G: Game, A: Agent<Game=G>, B: Agent<Game=G>> ManagedContest<'a, G, A, B> {
    /// Creates a new ManagedContest with the initial state and agents.
    /// Note that agent A always starts the game.
    pub fn new(initial_state: G::State, agent_a: &'a mut A, agent_b: &'a mut B) -> Self {
        ManagedContest {
            state: initial_state.clone(),
            history: GameHistory::new(B::identifier(), A::identifier(), initial_state),
            agent_a,
            agent_b,
        }
    }

    pub fn new_with_ids(
        initial_state: G::State,
        agent_a: &'a mut A,
        agent_b: &'a mut B,
        agent_a_identifier: String,
        agent_b_identifier: String,
    ) -> Self {
        ManagedContest {
            state: initial_state.clone(),
            history: GameHistory::new(agent_b_identifier, agent_a_identifier, initial_state),
            agent_a,
            agent_b,
        }
    }

    /// Returns the result of the game or None if the game is not over.
    pub fn game_result(&self) -> Option<G::GameResult> {
        self.state.game_result()
    }

    /// Returns a reference to Agent A.
    pub fn agent_a(&self) -> &A {
        self.agent_a
    }

    /// Returns a reference to Agent B.
    pub fn agent_b(&self) -> &B {
        self.agent_b
    }

    /// Returns a reference to the current state of the game.
    pub fn state(&self) -> &G::State {
        &self.state
    }

    /// Plays out a full game between the two agents and returns the result.
    pub fn play(&mut self) -> Option<G::GameResult> {
        for _ in &mut *self {}
        self.game_result()
    }

    pub fn history(&self) -> &GameHistory<G> {
        &self.history
    }
}

impl<'a, G: Game, A: Agent<Game=G>, B: Agent<Game=G>> Iterator
for &mut ManagedContest<'a, G, A, B>
{
    type Item = (G::State, G::Action, G::State);

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
        if self.state.is_terminal() {
            return None;
        }

        let old_state = self.state.clone();

        // increment the turn - INVARIANT IS BROKEN
        self.state = self.state.advance_ply();

        let agent_start = std::time::Instant::now();

        // ply 1 is the first action, so agent A starts
        let action = if self.state.ply() % 2 == 1 {
            self.agent_a.recommend_move(&self.state)
        } else {
            self.agent_b.recommend_move(&self.state)
        };

        let agent_time = agent_start.elapsed();

        // apply the action, finishing the turn - INVARIANT IS RESTORED
        self.state = self.state.apply_action(&action);

        // update the history
        self.history
            .add_turn(action.clone(), self.state.clone(), agent_time);

        Some((old_state, action, self.state.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agents::monke_agent::MonkeAgent;
    use crate::games::counting_game::*;

    #[test]
    fn test_history() {
        let mut agent1: MonkeAgent<CountingGame> = MonkeAgent::default();
        let mut agent2: MonkeAgent<CountingGame> = MonkeAgent::default();

        let mut contest = ManagedContest::new(
            CountingGame::initial_state(),
            &mut agent1,
            &mut agent2);
        contest.play();
        let history = contest.history();
        assert_ne!(history.turns().len(), 0);
    }
}
