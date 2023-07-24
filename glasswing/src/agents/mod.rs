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
    actions: &mut [G::Action],
    evaluator: &E,
) {
    actions.sort_by_cached_key(|action| {
        //use -value, such that we have descending order
        OrderedFloat(-evaluator.action_heuristic(state, action))
    });
}

#[cfg(test)]
mod tests {
    use crate::agents::sort_actions;
    use crate::core::{Game, State};
    use crate::games::counting_game::{CountingAction, CountingGame, CountingGameEvaluator};

    #[test]
    fn test_sort() {
        let state = CountingGame::initial_state();
        let mut actions = state.actions().into_iter().collect::<Vec<_>>();
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
        sort_actions(&state, &mut actions, &eval);

        // Heuristic maximises increment
        assert_eq!(
            actions,
            vec![
                CountingAction { increment: 3 },
                CountingAction { increment: 2 },
                CountingAction { increment: 1 }
            ],
            "Unexpected order after sorting by heuristic"
        )
    }
}
