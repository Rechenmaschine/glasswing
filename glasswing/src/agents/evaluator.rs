use crate::core::{Game, GwState};

// Evaluates a game state, returning a score.
pub trait Evaluator<G: Game> {
    /// Evaluate the state, returning a score.
    fn evaluate(&mut self, state: &G::State) -> G::EvalType;

    /// Assumes that the given action is legal in the given state.
    #[inline]
    fn evaluate_action(&mut self, state: &G::State, action: &G::Action) -> G::EvalType {
        self.evaluate(&state.apply_action(action))
    }
}
