use std::any::type_name;
use std::time::Duration;
use crate::core::Agent;
use crate::Game;

pub struct Player<A: Agent> {
    agent: A,
    description: &'static str,
    time_limit: Duration,
}

#[derive(Debug)]
pub enum BuilderError {
    MissingAttribute(&'static str)
}

impl<A: Agent> Player<A> {
    pub fn recommend_move(&mut self, state: &<A::Game as Game>::State) -> <A::Game as Game>::Action {
        self.agent.recommend_move_with_time(state, self.time_limit)
    }

    pub fn description(&self) -> &str {
        &self.description
    }


    pub fn time_limit(&self) -> Duration {
        self.time_limit
    }
}

/// implement builder pattern for Player
pub struct PlayerBuilder<A: Agent> {
    agent: Option<A>,
    description: Option<&'static str>,
    time_limit: Option<Duration>,
}

impl<A: Agent> PlayerBuilder<A> {

    /// Create a new PlayerBuilder
    pub fn new() -> Self {
        PlayerBuilder {
            agent: None,
            description: None,
            time_limit: None,
        }
    }

    /// Set the agent used for move recommendation in the player
    pub fn agent(mut self, agent: A) -> Self {
        self.agent = Some(agent);
        self
    }

    /// Set the name of the player. If not set, the name will be the type name of the agent
    pub fn name(mut self, name: &'static str) -> Self {
        self.description = Some(name);
        self
    }

    /// Set the time limit upper bound for the agent. If not set, the agent
    /// will have unlimited time for each move
    pub fn time_limit(mut self, time_limit: Duration) -> Self {
        self.time_limit = Some(time_limit);
        self
    }

    /// Build the player. Returns an error if a required attribute is missing.
    pub fn build(self) -> Result<Player<A>, BuilderError> {
        if let Some(agent) = self.agent {
            Ok(Player {
                agent,
                description: self.description.unwrap_or_else(|| type_name::<A>()),
                time_limit: self.time_limit.unwrap_or(Duration::MAX),
            })
        } else {
            Err(BuilderError::MissingAttribute("Agent not set"))
        }
    }
}