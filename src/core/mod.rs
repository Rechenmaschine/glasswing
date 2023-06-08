pub mod bridge;
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
