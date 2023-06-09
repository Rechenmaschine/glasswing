pub mod contest;
pub mod game_history;
pub mod player;
pub mod traits;

pub use contest::*;
pub use traits::*;

#[derive(Debug)]
pub enum BuilderError {
    MissingAttribute(&'static str),
}

// Agent error
#[derive(Debug)]
pub enum Error {
    Timeout,
    InvalidAction,
    IllegalAction,
    InvalidState,
    NoAvailableActions,
    TimeLimitExceeded,
    EvaluationError,
    GameNotOver,
    Other(&'static str),
}
