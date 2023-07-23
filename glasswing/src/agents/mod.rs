pub mod minimax_agent;
pub mod negamax_agent;
pub mod random_agent;
pub mod simple_agent;

pub use minimax_agent::*;
pub use random_agent::*;
pub use simple_agent::*;

use crate::core::{Evaluator, Game};
use ordered_float::OrderedFloat;

/// Utility function to sort actions in descending order of heuristic value
/// according to the given evaluator
fn sort_actions<G: Game, E: Evaluator<G>>(
    state: &G::State,
    mut actions: Vec<G::Action>,
    evaluator: &E,
) -> Vec<G::Action> {
    actions.sort_by_cached_key(|action| {
        //use -value, such that we have descending order
        OrderedFloat(-evaluator.action_heuristic(state, action))
    });
    actions
}

#[cfg(test)]
mod tests {
    use crate::agents::sort_actions;
    use crate::core::{Game, State};
    use crate::games::counting_game::{CountingAction, CountingGame, CountingGameEvaluator};

    #[test]
    fn test_sort() {
        let state = CountingGame::initial_state();
        let actions = state.actions();
        let eval = CountingGameEvaluator;

        assert_eq!(
            actions,
            vec![
                CountingAction { increment: 1 },
                CountingAction { increment: 2 },
                CountingAction { increment: 3 }
            ],
            "Implementation changed, please update test"
        );
        let sorted = sort_actions(&state, actions, &eval);

        // Heuristic maximises increment
        assert_eq!(
            sorted,
            vec![
                CountingAction { increment: 3 },
                CountingAction { increment: 2 },
                CountingAction { increment: 1 }
            ],
            "Unexpected order after sorting by heuristic"
        )
    }
}
