use crate::core::Game;
use std::fmt::Debug;

pub trait GwGameResult<G: Game>
where
    Self: Sized + Clone + Debug,
    G: Game<GameResult = Self>,
{
    fn winner(&self) -> Option<G::Team>;

    fn is_draw(&self) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameResult<G: Game>
where
    G: Game,
    G::Team: Debug,
{
    Win(G::Team),
    Draw,
}

impl<G> GwGameResult<G> for GameResult<G>
where
    Self: Sized + Clone + Debug,
    G: Game<GameResult = Self>,
{
    #[inline]
    fn winner(&self) -> Option<G::Team> {
        match self {
            GameResult::Win(team) => Some(team.clone()),
            GameResult::Draw => None,
        }
    }

    #[inline]
    fn is_draw(&self) -> bool {
        match self {
            GameResult::Win(_) => false,
            GameResult::Draw => true,
        }
    }
}

impl<G: Game<GameResult = GameResult<G>>> std::fmt::Display for GameResult<G>
where
    G: Game,
    G::Team: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GameResult::Win(team) => write!(f, "Win({})", team),
            GameResult::Draw => write!(f, "Draw"),
        }
    }
}
