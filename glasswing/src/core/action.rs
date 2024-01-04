use crate::core::game::Game;
use std::fmt::Debug;

pub trait GwAction<G>
where
    Self: Sized + Clone + Debug + Eq + PartialEq,
    G: Game<Action = Self>,
{
}
