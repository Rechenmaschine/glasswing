use crate::core::GwTeam;
use std::fmt;

pub trait GwGameResult<T: GwTeam>: Sized + Clone + fmt::Debug {
    fn winner(&self) -> Option<T>;

    fn is_draw(&self) -> bool {
        self.winner().is_none()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameResult<T: GwTeam> {
    Win(T),
    Draw,
}

impl<T: GwTeam> GwGameResult<T> for GameResult<T> {
    fn winner(&self) -> Option<T> {
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

impl<T: GwTeam + fmt::Display> fmt::Display for GameResult<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameResult::Win(team) => {
                write!(f, "{}", team)
            }
            GameResult::Draw => {
                write!(f, "Draw")
            }
        }
    }
}
