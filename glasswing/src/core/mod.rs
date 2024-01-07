use std::fmt::Debug;
pub mod game;
pub mod game_result;
pub mod state;
pub mod team;

pub use game::*;
pub use game_result::*;
pub use state::*;
pub use team::*;

#[derive(Debug, thiserror::Error)]
pub enum MatchError<G>
where
    G: Game,
    G::State: Debug,
{
    #[error("No available actions for state {0:?}")]
    NoAvailableActions(G::State),
}
