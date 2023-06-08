use crate::core::Agent;
use crate::core::Game;
use std::time::Duration;

/// A bridge is an adapter between the contest and the agent.
/// It handles the communication between the two.
/// It forwards the calls to the agent and handles errors that arise.
/// It should also handle threading and time limits on the agent side.
/// It can be used to implement pondering and can be used to add custom logging to the game.
pub trait Bridge<A: Agent> {
    fn recommend_action(
        &mut self,
        state: &<A::Game as Game>::State,
        time_limit: Duration,
    ) -> <A::Game as Game>::Action;
}

// This is a convenience implementation of the Bridge trait for agents, such that
// they can be directly passed to the contest.
impl<A: Agent> Bridge<A> for A {
    fn recommend_action(
        &mut self,
        state: &<A::Game as Game>::State,
        time_limit: Duration,
    ) -> <A::Game as Game>::Action {
        self.recommend_move_with_time(state, time_limit)
    }
}
