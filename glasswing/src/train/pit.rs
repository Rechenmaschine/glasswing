use crate::agents::Agent;
use crate::core::{Game, GwState};

#[allow(non_snake_case)]
pub struct Pit<G, A, B>
where
    G: Game,
{
    agentA: A,
    agentB: B,
    turn: usize,
    state: G::State,
}

impl<G, A, B> Pit<G, A, B>
where
    G: Game,
    A: Agent<G>,
    B: Agent<G>,
{
    #[allow(non_snake_case)]
    pub fn new(agentA: A, agentB: B, initial: G::State) -> Self {
        Pit {
            agentA,
            agentB,
            turn: 0,
            state: initial,
        }
    }

    pub fn playout(&mut self) -> Option<G::GameResult> {
        for _ in &mut *self {}
        self.state.game_result()
    }

    pub fn game_result(&self) -> Option<G::GameResult> {
        self.state.game_result()
    }

    pub fn state(&self) -> &G::State {
        &self.state
    }

    #[allow(non_snake_case)]
    pub fn agentA(&self) -> &A {
        &self.agentA
    }

    #[allow(non_snake_case)]
    pub fn agentA_mut(&mut self) -> &mut A {
        &mut self.agentA
    }

    #[allow(non_snake_case)]
    pub fn agentB(&self) -> &B {
        &self.agentB
    }

    #[allow(non_snake_case)]
    pub fn agentB_mut(&mut self) -> &mut B {
        &mut self.agentB
    }
}

impl<G, A, B> Iterator for Pit<G, A, B>
where
    G: Game,
    A: Agent<G>,
    B: Agent<G>,
{
    /// (previous state, action, post state)
    type Item = (G::State, G::Action, G::State);

    fn next(&mut self) -> Option<Self::Item> {
        if self.state.is_terminal() {
            return None;
        }

        let agent: &mut dyn Agent<G> = if self.turn % 2 == 0 {
            &mut self.agentA
        } else {
            &mut self.agentB
        };
        let action = agent.select_action(&self.state).unwrap();

        let pre = self.state.clone();
        self.state = self.state.apply_action(&action);
        self.turn += 1;

        Some((pre, action, self.state.clone()))
    }
}
