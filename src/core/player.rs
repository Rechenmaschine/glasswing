use crate::core::bridge::Bridge;
use crate::core::{Agent, BuilderError, Error, Game};
use std::any::type_name;
use std::time::Duration;

pub struct Player<A: Agent, Br: Bridge<A>> {
    agent: Br,
    name: &'static str,
    time_limit: Duration,
    _marker: std::marker::PhantomData<A>,
}

impl<A: Agent, Br: Bridge<A>> Player<A, Br> {
    pub fn recommend_action(
        &mut self,
        state: &<A::Game as Game>::State,
    ) -> Result<<A::Game as Game>::Action, Error> {
        self.agent.recommend_action(state, self.time_limit)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn time_limit(&self) -> Duration {
        self.time_limit
    }
}

/// implement builder pattern for Player
pub struct PlayerBuilder<A: Agent, Br: Bridge<A>> {
    agent: Option<Br>,
    name: Option<&'static str>,
    time_limit: Option<Duration>,
    _marker: std::marker::PhantomData<A>,
}

impl<A: Agent, Br: Bridge<A>> PlayerBuilder<A, Br> {
    /// Create a new PlayerBuilder
    pub fn new() -> Self {
        PlayerBuilder {
            agent: None,
            name: None,
            time_limit: None,
            _marker: Default::default(),
        }
    }

    /// Set the agent used for move recommendation in the player
    pub fn agent(mut self, agent: Br) -> Self {
        self.agent = Some(agent);
        self
    }

    /// Set the name of the player. If not set, the name will be the type name of the agent
    pub fn name(mut self, name: &'static str) -> Self {
        self.name = Some(name);
        self
    }

    /// Set the time limit upper bound for the agent. If not set, the agent
    /// will have unlimited time for each move
    pub fn time_limit(mut self, time_limit: Duration) -> Self {
        self.time_limit = Some(time_limit);
        self
    }

    /// Build the player. Returns an error if a required attribute is missing.
    pub fn build(self) -> Result<Player<A, Br>, BuilderError> {
        if let Some(agent) = self.agent {
            Ok(Player {
                agent,
                name: self.name.unwrap_or_else(|| type_name::<Br>()),
                time_limit: self.time_limit.unwrap_or(Duration::MAX),
                _marker: Default::default(),
            })
        } else {
            Err(BuilderError::MissingAttribute("Agent not set"))
        }
    }
}
