use std::fmt;

pub trait GwTeam: Sized + Clone + Eq + PartialEq + fmt::Debug {
    fn opponent(&self) -> Self;

    #[inline]
    fn nth(&self, n: usize) -> Self {
        if n % 2 == 0 {
            self.clone()
        } else {
            self.opponent()
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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Team {
    One,
    Two,
}

impl GwTeam for Team {
    #[inline]
    fn opponent(&self) -> Self {
        match self {
            Team::One => Team::Two,
            Team::Two => Team::One,
        }
    }
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

impl fmt::Debug for Team {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
