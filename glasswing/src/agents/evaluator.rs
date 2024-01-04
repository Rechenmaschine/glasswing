use crate::core::{Game, GwState};

/// Evaluators provide functions for evaluating a game state. These evaluations
/// are always **relative** to a team. That is, the evaluator should return
/// a score that is positive if the given team is winning, and negative if
/// the given team is losing, regardless of the team to move.
///
///
/// # Symmetry
/// In general, evaluators should return symmetric scores. That is,
/// for all States *S* and teams *A* and *B*, it should hold that
///
/// `eval(S, A) = -eval(S, B)`
///
/// Not ensuring this can lead to suboptimal play!
///
/// However, if you really know what you're doing, asymmetry can be useful. See
/// [https://stackoverflow.com/questions/43813955/can-negamax-use-an-asymmetric-evaluation-function](this StackOverflow post)
/// for more information.
pub trait Evaluator<G: Game> {
    /// Evaluate the state relative to the current team, returning a score.
    fn evaluate(&mut self, state: &G::State) -> G::EvalType {
        self.evaluate_for(state, &state.team_to_move())
    }

    /// Evaluate the state relative to the given team, returning a score.
    fn evaluate_for(&mut self, state: &G::State, team: &G::Team) -> G::EvalType;

    /// Evaluate the state relative to the given team, returning a score.
    ///
    /// # Assumptions
    /// - The given action is legal in the given state.
    #[inline]
    fn evaluate_action_for(
        &mut self,
        state: &G::State,
        action: &G::Action,
        team: &G::Team,
    ) -> G::EvalType {
        self.evaluate_for(&state.apply_action(action), team)
    }
}
