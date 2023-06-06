use crate::core::Game;
use serde::{Deserialize, Serialize};
use std::ops::Index;
use std::time::Duration;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Turn<G: Game> {
    action: G::Action,
    state: G::State,
    agent_time: Duration,
}

/// The execution history of a game played between two agents.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(bound = "for<'de2> G: Deserialize<'de2>")]
pub struct GameHistory<G: Game> {
    agent_a_id: String,
    agent_b_id: String,
    initial_state: G::State,
    turns: Vec<Turn<G>>,
}

impl<G: Game> GameHistory<G> {
    pub fn new(agent_a_id: String, agent_b_id: String, initial_state: G::State) -> Self {
        GameHistory {
            agent_a_id,
            agent_b_id,
            initial_state,
            turns: Vec::new(),
        }
    }

    pub fn turns(&self) -> &[Turn<G>] {
        &self.turns
    }

    pub fn add_turn(&mut self, action: G::Action, state: G::State, agent_time: Duration) {
        self.turns.push(Turn {
            action,
            state,
            agent_time,
        });
    }
}

impl<G: Game> Index<usize> for GameHistory<G> {
    type Output = Turn<G>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.turns[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agents::monke_agent::MonkeAgent;
    use crate::core::Contest;
    use crate::games::counting_game::CountingGame;

    #[test]
    fn test_serde() {
        let mut agent1: MonkeAgent<CountingGame> = MonkeAgent::default();
        let mut agent2: MonkeAgent<CountingGame> = MonkeAgent::default();

        let mut history: GameHistory<CountingGame> = GameHistory::new(
            "agent_1".to_string(),
            "agent_2".to_string(),
            CountingGame::initial_state(),
        );

        let mut contest = Contest::new(CountingGame::initial_state(), &mut agent1, &mut agent2);
        while let Some((_old, action, curr)) = (&mut contest).next() {
            history.add_turn(action, curr, Duration::from_secs(0));
        }

        let ser = serde_json::to_string(&history).expect("Failed to serialize");
        serde_json::from_str::<GameHistory<CountingGame>>(&ser).expect("Failed to deserialize");
    }
}
