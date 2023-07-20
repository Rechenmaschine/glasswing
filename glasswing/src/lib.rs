pub mod agents;
pub mod core;

#[cfg(tournaments)]
pub mod tournaments;

#[cfg(perft)]
pub mod perft;

pub mod prelude {
    pub use crate::agents::*;
    pub use crate::core::*;
}

#[cfg(test)]
mod games;
