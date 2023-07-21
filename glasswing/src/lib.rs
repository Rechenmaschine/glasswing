pub mod agents;
pub mod core;

/// Collection of tournament implementations.
#[cfg(feature = "tournaments")]
pub mod tournaments;

/// Perft functions
pub mod perft;

pub mod prelude {
    pub use crate::agents::*;
    pub use crate::core::*;
}

#[cfg(test)]
mod games;
