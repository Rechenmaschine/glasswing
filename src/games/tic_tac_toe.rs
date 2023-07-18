use std::fmt;
use std::fmt::Formatter;
use anyhow::Error;
use crate::core::{Action, Evaluator, Game, GameResult, State, Team};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct TicTacToe<const N: usize>;

impl<const N: usize> Game for TicTacToe<N> {
    type State = TicTacToeState<N>;
    type Action = TicTacToeAction;
    type Team = TicTacToeTeam;

    type GameResult = TicTacToeResult;
    const NAME: &'static str = "TicTacToe";

    fn initial_state() -> Self::State {
        TicTacToeState {
            board: [[None; N]; N],
            turn: 0,
        }
    }

    fn starting_team() -> Self::Team {
        TicTacToeTeam::X
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct TicTacToeState<const N: usize> {
    board: [[Option<TicTacToeTeam>; N]; N],
    turn: usize,
}

// implement Debug for TicTacToeState using display
impl<const N: usize> fmt::Debug for TicTacToeState<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl<const N: usize> State<TicTacToe<N>> for TicTacToeState<N> {
    fn actions(&self) -> Vec<TicTacToeAction> {
        let mut actions = vec![];
        for (x, row) in self.board.iter().enumerate() {
            for (y, tile) in row.iter().enumerate() {
                if tile.is_none() {
                    actions.push(TicTacToeAction::new(x, y));
                }
            }
        }
        actions
    }

    fn turn(&self) -> usize {
        self.turn
    }

    fn apply_action(&self, action: &TicTacToeAction) -> Self {
        let mut new_board = self.board.clone();
        new_board[action.col][action.row] = Some(self.team_to_move());
        Self {
            board: new_board,
            turn: self.turn + 1,
        }
    }

    fn is_terminal(&self) -> bool {
        // check if turn is greater than number of tiles
        return self.game_result().is_some();
    }

    fn game_result(&self) -> Option<TicTacToeResult> {
        // check rows
        for y in 0..N {
            if let Some(first_tile) = self.board[y][0] {
                for x in 0..N {
                    if let Some(tile) = self.board[y][x] {
                        if tile != first_tile {
                            break;
                        }
                        if x == N - 1 {
                            return Some(TicTacToeResult::Winner(tile));
                        }
                    } else {
                        break;
                    }
                }
            }
        }

        // check columns
        for x in 0..N {
            if let Some(first_tile) = self.board[0][x] {
                for y in 0..N {
                    if let Some(tile) = self.board[y][x] {
                        if tile != first_tile {
                            break;
                        }
                        if y == N - 1 {
                            return Some(TicTacToeResult::Winner(tile));
                        }
                    } else {
                        break;
                    }
                }
            }
        }

        // check diagonals
        if let Some(first_tile) = self.board[0][0] {
            for i in 0..N {
                if let Some(tile) = self.board[i][i] {
                    if tile != first_tile {
                        break;
                    }
                    if i == N - 1 {
                        return Some(TicTacToeResult::Winner(tile));
                    }
                } else {
                    break;
                }
            }
        }

        if let Some(first_tile) = self.board[0][N - 1] {
            for i in 0..N {
                if let Some(tile) = self.board[i][N - 1 - i] {
                    if tile != first_tile {
                        break;
                    }
                    if i == N - 1 {
                        return Some(TicTacToeResult::Winner(tile));
                    }
                } else {
                    break;
                }
            }
        }

        // check whether board is full
        for row in self.board.iter() {
            for tile in row.iter() {
                if tile.is_none() {
                    return None;
                }
            }
        }
        return Some(TicTacToeResult::Draw);
    }
}

impl<const N: usize> fmt::Display for TicTacToeState<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Turn: {}", self.turn)?;
        // write team
        writeln!(f, "Team: {:?}", self.team_to_move())?;
        for row in self.board.iter() {
            for tile in row.iter() {
                match tile {
                    Some(TicTacToeTeam::X) => write!(f, "X")?,
                    Some(TicTacToeTeam::O) => write!(f, "O")?,
                    None => write!(f, ".")?,
                }
                write!(f, "  ")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct TicTacToeAction {
    col: usize,
    row: usize,
}

impl TicTacToeAction {
    fn new(col: usize, row: usize) -> Self {
        TicTacToeAction { col, row }
    }
}

impl<const N: usize> Action<TicTacToe<N>> for TicTacToeAction {}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TicTacToeTeam {
    X,
    O,
}

impl<const N: usize> Team<TicTacToe<N>> for TicTacToeTeam {
    fn next(&self) -> Self {
        match self {
            TicTacToeTeam::X => TicTacToeTeam::O,
            TicTacToeTeam::O => TicTacToeTeam::X,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum TicTacToeResult {
    Winner(TicTacToeTeam),
    Draw,
}

impl<const N: usize> GameResult<TicTacToe<N>> for TicTacToeResult {
    fn winner(&self) -> Option<TicTacToeTeam> {
        match self {
            TicTacToeResult::Winner(team) => Some(*team),
            TicTacToeResult::Draw => None,
        }
    }

    fn is_draw(&self) -> bool {
        match self {
            TicTacToeResult::Winner(_) => false,
            TicTacToeResult::Draw => true,
        }
    }
}

// TTT Evaluator
struct TicTacToeEvaluator;

impl<const N: usize> Evaluator<TicTacToe<N>> for TicTacToeEvaluator {
    fn evaluate(&self, state: &TicTacToeState<N>) -> Result<f32, Error> {
        if let Some(result) = state.game_result() {
            match result {
                TicTacToeResult::Winner(team) => {
                    if team == TicTacToe::<N>::starting_team() {
                        Ok(100.0)
                    } else {
                        Ok(-100.0)
                    }
                }
                TicTacToeResult::Draw => {
                    Ok(0.0)
                }
            }
        } else {
            // Non-terminal state: return heuristic
            Ok(0.0)
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused)]

    use std::time::Duration;
    use log::{info, debug, error, warn, trace};
    use pretty_env_logger::env_logger::builder;
    use rand::rngs::OsRng;
    use crate::agents::minimax_agent::MiniMaxAgent;
    use crate::agents::random_agent::RandomAgent;
    use crate::agents::simple_agent::SimpleAgent;
    use crate::core::{Agent, Match};
    use crate::games::tic_tac_toe::TicTacToeResult::{Draw, Winner};
    use crate::games::tic_tac_toe::TicTacToeTeam::{X, O};
    use super::*;

    #[test]
    fn test_terminal_state() {
        type TTT = TicTacToe<3>;

        let x = Some(TicTacToeTeam::X);
        let o = Some(TicTacToeTeam::O);

        let state = TicTacToeState {
            board: [
                [None, None, None],
                [None, None, None],
                [None, None, None],
            ],
            turn: 0,
        };

        assert_eq!(state.game_result(), None);
        assert!(!state.is_terminal());

        let state = TicTacToeState {
            board: [
                [None, None, x],
                [None, x, None],
                [x, None, None],
            ],
            turn: 0,
        };

        assert_eq!(state.game_result(), Some(Winner(X)));
        assert!(state.is_terminal());

        let state = TicTacToeState {
            board: [
                [o, None, o],
                [None, o, None],
                [o, None, o],
            ],
            turn: 0,
        };

        assert_eq!(state.game_result(), Some(Winner(O)));
        assert!(state.is_terminal());

        let state = TicTacToeState {
            board: [
                [x, x, x],
                [o, o, x],
                [x, o, o],
            ],
            turn: 0,
        };

        assert_eq!(state.game_result(), Some(Winner(X)));
        assert!(state.is_terminal());

        let state = TicTacToeState {
            board: [
                [x, x, None],
                [o, None, None],
                [x, None, None],
            ],
            turn: 0,
        };

        assert_eq!(state.game_result(), None);
        assert!(!state.is_terminal());

        //X  X  O
        //X  X  O
        //.  O  .
        let state = TicTacToeState {
            board: [
                [x, x, o],
                [x, x, o],
                [None, o, None],
            ],
            turn: 0,
        };

        assert_eq!(state.game_result(), None);
        assert!(!state.is_terminal());

        //X  X  O
        //X  X  O
        //O  O  O

        let state = TicTacToeState {
            board: [
                [x, x, o],
                [x, x, o],
                [o, o, o],
            ],
            turn: 0,
        };

        assert_eq!(state.game_result(), Some(Winner(O)));
        assert!(state.is_terminal());
    }

    #[test]
    fn test_evals() {
        let evaluator = TicTacToeEvaluator;
        let eval = |state: &TicTacToeState<3>| -> f32 {
            evaluator.evaluate(state).unwrap()
        };

        let state = TicTacToeState {
            board: [
                [None, None, None],
                [None, None, None],
                [None, None, None],
            ],
            turn: 0,
        };

        assert_eq!(eval(&state), 0.0);

        // state where X wins
        let state = TicTacToeState {
            board: [
                [Some(X), Some(X), Some(X)],
                [Some(O), Some(O), Some(X)],
                [Some(X), Some(O), Some(O)],
            ],
            turn: 9,
        };

        assert_eq!(eval(&state), 100.0);

        //same state but inverted
        let state = TicTacToeState {
            board: [
                [Some(O), Some(O), Some(O)],
                [Some(X), Some(X), Some(O)],
                [Some(O), Some(X), Some(X)],
            ],
            turn: 9,
        };

        assert_eq!(eval(&state), -100.0);
    }

    #[test]
    fn test() {
        // init logger
        builder().filter_level(log::LevelFilter::Debug).init();

        let mut wins_minimax = 0;
        let mut wins_random = 0;
        let mut draws = 0;

        for i in 0..10 {
            let minimax = MiniMaxAgent::new(10, TicTacToeEvaluator);
            let random = RandomAgent::new(OsRng::default());

            let mut match_: Match<TicTacToe<3>> = if i % 2 == 0 {
                Match::new(minimax, random)
            } else {
                Match::new(random, minimax)
            };

            match match_.playout() {
                Ok(result) => {
                    match result.game_result().expect("Game result should be present") {
                        Winner(winner) => {
                            if winner == X && i % 2 == 0 || winner == O && i % 2 == 1 {
                                wins_minimax += 1;
                                info!("Minimax won as team {:?}", winner)
                            } else if winner == O && i % 2 == 0 || winner == X && i % 2 == 1 {
                                wins_random += 1;
                                info!("Random won as team {:?}", winner)
                            } else {
                                unreachable!("Invalid state")
                            }
                        }
                        Draw => {
                            draws += 1;
                            info!("Draw")
                        }
                    }
                }
                Err(e) => {
                    error!("Error: {}", e);
                }
            }

            if i % 10 == 9 {
                info!("\n======= STATISTICS =======\nWins minimax: {}\nWins random: {}\nDraws: {}\n==========================", wins_minimax, wins_random, draws);
            }
        }
    }

    #[test]
    fn test2() {
        builder().filter_level(log::LevelFilter::Info).init();

        let mut wins_minimax = 0;
        let mut wins_random = 0;
        let mut draws = 0;

        for i in 0..10000 {
            let minimax = MiniMaxAgent::new(10, TicTacToeEvaluator);
            let random = RandomAgent::new(OsRng::default());

            let mut match_: Match<TicTacToe<3>> = Match::new(minimax, random);

            match match_.playout() {
                Ok(result) => {
                    match result.game_result().expect("Game result should be present") {
                        Winner(winner) => {
                            match winner {
                                X => {
                                    wins_minimax += 1;
                                    info!("minimax won as team {:?}", winner);
                                }
                                O => {
                                    wins_random += 1;
                                    info!("random won as team {:?}", winner);
                                }
                            }
                        }
                        Draw => {
                            draws += 1;
                            info!("Draw")
                        }
                    }
                }
                Err(e) => {
                    error!("Error: {}", e);
                }
            }

            if i % 10 == 9 {
                info!("\n======= STATISTICS =======\nWins minimax: {}\nWins random: {}\nDraws: {}\n==========================", wins_minimax, wins_random, draws);
            }
        }
    }
}