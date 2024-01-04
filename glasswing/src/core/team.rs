use crate::core::Game;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Polarity {
    Positive = 1,
    Negative = -1,
}

impl Polarity {
    #[inline(always)]
    pub fn flip(&self) -> Self {
        match self {
            Polarity::Positive => Polarity::Negative,
            Polarity::Negative => Polarity::Positive,
        }
    }

    #[inline(always)]
    pub fn sign(&self) -> i32 {
        match self {
            Polarity::Positive => 1,
            Polarity::Negative => -1,
        }
    }
}

impl Team {
    #[inline(always)]
    pub fn opponent(&self) -> Self {
        match self {
            Team::One => Team::Two,
            Team::Two => Team::One,
        }
    }
}

pub trait GwTeam<G>
where
    Self: Sized + Clone + Debug + Eq + PartialEq,
    G: Game<Team = Self>,
{
    fn next(&self) -> Self;

    fn polarity(&self) -> Polarity;

    #[inline]
    fn nth(&self, n: usize) -> Self {
        let mut team = self.clone();
        for _ in 0..n {
            team = team.next();
        }
        team
    }
}

/// Team for a two player game.
/// Why is this not an enum? Performance.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Team {
    One,
    Two,
}

impl<G: Game<Team = Self>> GwTeam<G> for Team {
    #[inline(always)]
    fn next(&self) -> Self {
        match self {
            Team::One => Team::Two,
            Team::Two => Team::One,
        }
    }

    #[inline(always)]
    fn polarity(&self) -> Polarity {
        match self {
            Team::One => Polarity::Positive,
            Team::Two => Polarity::Negative,
        }
    }
}

impl std::fmt::Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Team::One => {
                write!(f, "One")
            }
            Team::Two => {
                write!(f, "Two")
            }
        }
    }
}

impl Debug for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Team::One => {
                write!(f, "Team(One)")
            }
            Team::Two => {
                write!(f, "Team(Two)")
            }
        }
    }
}
