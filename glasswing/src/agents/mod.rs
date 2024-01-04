pub mod agent;
pub mod evaluator;
pub mod functional_agent;
//pub mod minimax;
pub mod negamax;
pub mod random_agent;
pub mod simple_agent;
pub mod human_agent;

pub use agent::*;
pub use evaluator::*;
//pub use minimax::MiniMax;
pub use negamax::NegaMax;
pub use random_agent::RandomAgent;
pub use simple_agent::SimpleAgent;
pub use human_agent::HumanAgent;

use crate::core::Game;

/// Utility function to stable sort actions in descending order of heuristic value
/// according to the given evaluator
fn sort_actions<G, E>(state: &G::State, actions: &mut [G::Action], evaluator: &mut E, for_team: &G::Team)
where
    G: Game,
    E: Evaluator<G>,
    G::EvalType: Ord,
{
    actions.sort_by_cached_key(|action| {
        evaluator.evaluate_action_for(state, action, for_team)
    });
}
