pub mod agents;
pub mod core;
pub mod games;
pub mod ranking;

pub mod prelude{
    pub use crate::core::player::PlayerBuilder;
    pub use crate::core::{ContestBuilder, Game};
    pub use crate::games::counting_game::{CountingGame, CountingGameEvaluator};
}