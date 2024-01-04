use super::{GwAction, GwGameResult, GwState, GwTeam};
use std::fmt::Debug;

pub trait Game
where
    Self: Sized + Debug + 'static,
{
    type State: GwState<Self>;
    type Action: GwAction<Self>;
    type Team: GwTeam<Self>;
    type GameResult: GwGameResult<Self>;
    type EvalType;

    fn initial_state() -> Self::State;

    fn starting_team() -> Self::Team;
}
