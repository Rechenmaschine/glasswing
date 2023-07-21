pub mod pit;
pub mod serialization;
pub mod traits;
//pub mod game_history;

pub use pit::*;
pub use traits::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub enum TwoPlayerTeam {
    One,
    Two,
}

impl<G: Game<Team = TwoPlayerTeam>> Team<G> for TwoPlayerTeam {
    fn next(&self) -> Self {
        match self {
            TwoPlayerTeam::One => TwoPlayerTeam::Two,
            TwoPlayerTeam::Two => TwoPlayerTeam::One,
        }
    }

    fn polarity(&self) -> Polarity {
        match self {
            TwoPlayerTeam::One => Polarity::Positive,
            TwoPlayerTeam::Two => Polarity::Negative,
        }
    }
}

#[derive(Debug, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub enum TwoPlayerGameResult<G: Game> {
    Winner(G::Team),
    Draw,
}

// Manual implementation of Clone because of the generic type parameter
impl<G: Game> Clone for TwoPlayerGameResult<G> {
    fn clone(&self) -> Self {
        match self {
            TwoPlayerGameResult::Winner(team) => TwoPlayerGameResult::Winner(*team),
            TwoPlayerGameResult::Draw => TwoPlayerGameResult::Draw,
        }
    }
}

impl<G: Game> GameResult<G> for TwoPlayerGameResult<G> {
    fn winner(&self) -> Option<G::Team> {
        match self {
            TwoPlayerGameResult::Winner(team) => Some(*team),
            TwoPlayerGameResult::Draw => None,
        }
    }

    fn is_draw(&self) -> bool {
        match self {
            TwoPlayerGameResult::Winner(_) => false,
            TwoPlayerGameResult::Draw => true,
        }
    }
}
