use crate::core::traits::*;

/// A match between two agents in a game.
pub struct Contest<'a, G: Game, A: Agent<Game = G>, B: Agent<Game = G>> {
    state: G::State,    // The current state of the game.
    agent_a: &'a mut A, // Agent A participating in the contest.
    agent_b: &'a mut B, // Agent B participating in the contest.
}

impl<'a, G: Game, A: Agent<Game = G>, B: Agent<Game = G>> Contest<'a, G, A, B> {
    /// Creates a new contest with the initial state and agents.
    ///
    /// Note that agent A always starts the game.
    pub fn new(initial_state: G::State, agent_a: &'a mut A, agent_b: &'a mut B) -> Self {
        Contest {
            state: initial_state,
            agent_a,
            agent_b,
        }
    }

    /// Returns the result of the game.
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
}

impl<'a, G: Game, A: Agent<Game = G>, B: Agent<Game = G>> Iterator for &mut Contest<'a, G, A, B> {
    type Item = (G::State, G::Action, G::State);

    /// Advances the contest to the next state, calling each agent in turn
    /// to recommend an action and then yielding the action and resulting state.
    ///
    /// ## Usage:
    /// ```
    /// for (old_state, action, cur_state) in &mut contest {
    ///     ...
    /// }
    /// ```
    ///
    /// For access to the contest, use the following syntax instead:
    /// ```
    /// while let Some((old_state, action, cur_state)) = (&mut contest).next() {
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

        // ply 1 is the first action, so agent A starts
        let action = if self.state.ply() % 2 == 1 {
            self.agent_a.recommend_move(&self.state)
        } else {
            self.agent_b.recommend_move(&self.state)
        };

        // apply the action, finishing the turn - INVARIANT IS RESTORED
        self.state = self.state.apply_action(&action);

        Some((old_state, action, self.state.clone()))
    }
}
