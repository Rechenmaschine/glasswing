use std::ops::Index;
use std::time::Duration;
use crate::core::Game;

#[cfg(feature = "serde_support")]
pub use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Turn<G: Game> {
    action: G::Action,
    state: G::State,
    agent_time: Duration,
}

/// The execution history of a game played between two agents.
#[cfg_attr(
feature = "serde_support",
derive(Serialize, Deserialize),
serde(bound = "for<'de2> G: Deserialize<'de2>")
)]
#[derive(Clone, Debug)]
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

    #[cfg(feature = "serde_support")]
    pub fn save_to<P: AsRef<std::path::Path>>(&self, path: P) -> std::io::Result<()> {
        let mut file = std::fs::File::create(path)?;
        let json = serde_json::to_string(self)?;
        std::io::Write::write_all(&mut file, json.as_bytes())?;
        Ok(())
    }

    #[cfg(feature = "serde_support")]
    pub fn load_from<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<Self> {
        let file = std::fs::File::open(path)?;
        let history = serde_json::from_reader(file)?;
        Ok(history)
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
    use crate::agents::simple_agent::SimpleAgent;
    use crate::{ContestBuilder, PlayerBuilder};
    use crate::games::counting_game::CountingGame;

    #[test]
    #[cfg(feature = "serde_support")]
    fn test_serde() {
        let agent1: SimpleAgent<CountingGame> = SimpleAgent::default();
        let agent2: SimpleAgent<CountingGame> = SimpleAgent::default();

        let mut history: GameHistory<CountingGame> = GameHistory::new(
            "player_1".to_string(),
            "player_2".to_string(),
            CountingGame::initial_state(),
        );

        let mut contest = ContestBuilder::new()
            .initial_state(CountingGame::initial_state())
            .player_starts(PlayerBuilder::new().agent(agent1).build().unwrap())
            .plays_aginst(PlayerBuilder::new().agent(agent2).build().unwrap())
            .build()
            .unwrap();


        while let Some((_old, action, curr)) = (&mut contest).next() {
            history.add_turn(action, curr, Duration::from_secs(0));
        }

        let ser = serde_json::to_string(&history).expect("Failed to serialize");
        serde_json::from_str::<GameHistory<CountingGame>>(&ser).expect("Failed to deserialize");

        history
            .save_to("test.json")
            .expect("Failed to save history");

        GameHistory::<CountingGame>::load_from("test.json").expect("Failed to load history");

        std::fs::remove_file("test.json").expect("Failed to delete test.json");
    }
}
