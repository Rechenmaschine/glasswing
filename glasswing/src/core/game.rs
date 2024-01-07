use super::{GwGameResult, GwState, GwTeam};
use std::fmt::Debug;

pub trait Game: Sized + Debug + 'static {
    type State: GwState<Self>;
    type Action: Clone + Debug;
    type Team: GwTeam;
    type GameResult: GwGameResult<Self::Team>;
    type EvalType;

    fn initial_state() -> Self::State;
}
