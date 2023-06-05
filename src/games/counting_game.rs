use crate::{Action, Game, GameResult, State, Team};
use crate::core::Evaluator;

/// A game, where each player can add 0, 1 or 2 to a total. The player who counts to 100 first, wins.
pub struct CountingGame;

impl Game for CountingGame {
    type State = CountingState;
    type Action = CountingAction;
    type Team = CountingTeam;
    type GameResult = CountingGameResult;

    fn initial_state() -> Self::State {
        CountingState { total: 0, turn: 0 }
    }
}

// CountingGameResult
#[derive(Copy, Clone, Debug)]
pub enum CountingGameResult {
    Winner(CountingTeam),
    Draw,
}

impl GameResult<CountingGame> for CountingGameResult {
    fn winner(&self) -> Option<CountingTeam> {
        match self {
            CountingGameResult::Winner(team) => Some(*team),
            CountingGameResult::Draw => None,
        }
    }

    fn is_draw(&self) -> bool {
        match self {
            CountingGameResult::Winner(_) => false,
            CountingGameResult::Draw => true,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum CountingTeam {
    One,
    Two,
}

impl Team<CountingGame> for CountingTeam {
    fn next(&self) -> Self {
        match self {
            CountingTeam::One => CountingTeam::Two,
            CountingTeam::Two => CountingTeam::One,
        }
    }
}

#[derive(Clone, Debug)]
pub struct CountingState {
    total: u8,
    turn: usize,
}

impl CountingState {
    pub fn total(&self) -> u8 {
        self.total
    }
}

impl State<CountingGame> for CountingState {
    fn actions(&self) -> Vec<CountingAction> {
        vec![
            CountingAction { increment: 1 },
            CountingAction { increment: 2 },
            CountingAction { increment: 3 },
        ]
    }

    fn current_team(&self) -> CountingTeam {
        if self.turn % 2 == 0 {
            CountingTeam::One
        } else {
            CountingTeam::Two
        }
    }

    fn ply(&self) -> usize {
        self.turn
    }

    fn apply_action(&self, action: &CountingAction) -> Self {
        Self {
            total: self.total + action.increment,
            turn: self.turn
        }
    }

    fn advance_ply(&self) -> Self {
        Self {
            total: self.total,
            turn: self.turn + 1,
        }
    }

    fn is_terminal(&self) -> bool {
        self.total >= 21
    }

    fn game_result(&self) -> Option<CountingGameResult> {
        if self.total >= 21 {
            Some(CountingGameResult::Winner(self.current_team()))
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct CountingAction {
    pub increment: u8,
}

impl Action<CountingGame> for CountingAction {}


pub struct CountingGameEvaluator;

impl Evaluator<CountingGame> for CountingGameEvaluator {
    fn evaluate(&self, state: &CountingState) -> f32 {
        if state.is_terminal() {
            state.game_result().expect("State marked as terminal, but no game result available").winner().map(|t| match t {
                CountingTeam::One => 100.0,
                CountingTeam::Two => -100.0,
            }).unwrap_or(0.0)
        } else {
            // the heuristic: the higher the score is, the better.
            state.total as f32 * match state.current_team() {
                CountingTeam::One => 1.0,
                CountingTeam::Two => -1.0,
            }
        }
    }
}