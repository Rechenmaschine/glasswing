use crate::core::{Game, GwState};

pub trait Evaluator<G: Game> {
    /// Evaluate the state relative to the current team, returning a score.
    //#[deprecated(note = "Use evaluate_for() instead.")]
    fn evaluate(&mut self, state: &G::State) -> G::EvalType{
        self.evaluate_for(state, &state.team_to_move())
    }


    /// Evaluate the state relative to the given team, returning a score.
    fn evaluate_for(&mut self, state: &G::State, team: &G::Team) -> G::EvalType;

    /// Evaluate the state relative to the given team, returning a score.
    ///
    /// # Assumptions
    /// - The given action is legal in the given state.
    #[inline]
    fn evaluate_action_for(&mut self, state: &G::State, action: &G::Action, team: &G::Team) -> G::EvalType {
        self.evaluate_for(&state.apply_action(action), team)
    }
}

/// Marker trait for evaluators that return symmetric scores.
/// That is, for all States *S* and teams *A* and *B*, it should hold that
///
/// `eval(S, A) = -eval(S, B)`
///
/// This trait is required for certain algorithms, such as [NegaMax], that
/// take advantage of this property.
///
/// [NegaMax]: crate::agents::NegaMax
pub trait SymmetricEvaluation<G: Game>: Evaluator<G> {}