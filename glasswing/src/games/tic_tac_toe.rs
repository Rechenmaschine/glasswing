use crate::core::TwoPlayerGameResult::{Draw, Winner};
use crate::core::TwoPlayerTeam::{One as X, Two as O};
use crate::core::{Action, Evaluator, Game, State, TwoPlayerGameResult, TwoPlayerTeam};
use anyhow::Error;
use std::fmt;
use std::fmt::Formatter;
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TicTacToe<const N: usize>;

type GameResult<const N: usize> = TwoPlayerGameResult<TicTacToe<N>>;

impl<const N: usize> Game for TicTacToe<N> {
    type State = TicTacToeState<N>;
    type Action = TicTacToeAction;
    type Team = TwoPlayerTeam;

    type GameResult = TwoPlayerGameResult<Self>;
    const NAME: &'static str = "TicTacToe";

    fn initial_state() -> Self::State {
        TicTacToeState {
            board: [[None; N]; N],
            turn: 0,
        }
    }

    fn starting_team() -> Self::Team {
        X
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct TicTacToeState<const N: usize> {
    board: [[Option<TwoPlayerTeam>; N]; N],
    turn: usize,
}

// implement Debug for TicTacToeState using display
impl<const N: usize> fmt::Debug for TicTacToeState<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl<const N: usize> State<TicTacToe<N>> for TicTacToeState<N> {
    type ActionIterator = std::vec::IntoIter<TicTacToeAction>;

    fn actions(&self) -> Self::ActionIterator {
        let mut actions = vec![];
        for (x, row) in self.board.iter().enumerate() {
            for (y, tile) in row.iter().enumerate() {
                if tile.is_none() {
                    actions.push(TicTacToeAction::new(x, y));
                }
            }
        }
        actions.into_iter()
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

    fn game_result(&self) -> Option<GameResult<N>> {
        // check rows
        for y in 0..N {
            if let Some(first_tile) = self.board[y][0] {
                for x in 0..N {
                    if let Some(tile) = self.board[y][x] {
                        if tile != first_tile {
                            break;
                        }
                        if x == N - 1 {
                            return Some(Winner(tile));
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
                            return Some(Winner(tile));
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
                        return Some(Winner(tile));
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
                        return Some(Winner(tile));
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
        return Some(Draw);
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
                    Some(X) => write!(f, "X")?,
                    Some(O) => write!(f, "O")?,
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
pub struct TicTacToeAction {
    col: usize,
    row: usize,
}

impl TicTacToeAction {
    fn new(col: usize, row: usize) -> Self {
        TicTacToeAction { col, row }
    }
}

impl<const N: usize> Action<TicTacToe<N>> for TicTacToeAction {}

// TTT Evaluator
pub struct TicTacToeEvaluator;

impl<const N: usize> Evaluator<TicTacToe<N>> for TicTacToeEvaluator {
    fn evaluate(&self, state: &TicTacToeState<N>) -> Result<f32, Error> {
        if let Some(result) = state.game_result() {
            match result {
                Winner(team) => {
                    if team == TicTacToe::<N>::starting_team() {
                        Ok(100.0)
                    } else {
                        Ok(-100.0)
                    }
                }
                Draw => Ok(0.0),
            }
        } else {
            // Non-terminal state: return heuristic
            Ok(self.heuristic(state))
        }
    }

    fn heuristic(&self, _: &TicTacToeState<N>) -> f32 {
        0.0
    }
}

//impl hash for TicTacToeState {
impl<const N: usize> Hash for TicTacToeState<N> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut hash = 0;
        let mut bit_count = 0u8;
        for row in self.board.iter() {
            for tile in row.iter() {
                match tile {
                    Some(X) => {
                        hash = hash << 2;
                        hash += 3;
                        bit_count += 2;
                    }
                    Some(O) => {
                        hash = hash << 2;
                        hash += 2;
                        bit_count += 2;
                    }
                    None => {
                        hash = hash << 2;
                        bit_count += 2;
                    }
                }
                if bit_count == 64 {
                    state.write_u64(hash);
                    hash = 0;
                    bit_count = 0;
                }
            }
        }
        state.write_u64(hash);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::TwoPlayerGameResult::Winner;
    use crate::core::TwoPlayerTeam::{One as X, Two as O};
    use crate::perft;
    //use pretty_env_logger::env_logger::builder;

    #[test]
    fn test_terminal_state() {
        let x = Some(X);
        let o = Some(O);

        let state = TicTacToeState {
            board: [[None, None, None], [None, None, None], [None, None, None]],
            turn: 0,
        };

        assert_eq!(state.game_result(), None);
        assert!(!state.is_terminal());

        let state = TicTacToeState {
            board: [[None, None, x], [None, x, None], [x, None, None]],
            turn: 0,
        };

        assert_eq!(state.game_result(), Some(Winner(X)));
        assert!(state.is_terminal());

        let state = TicTacToeState {
            board: [[o, None, o], [None, o, None], [o, None, o]],
            turn: 0,
        };

        assert_eq!(state.game_result(), Some(Winner(O)));
        assert!(state.is_terminal());

        let state = TicTacToeState {
            board: [[x, x, x], [o, o, x], [x, o, o]],
            turn: 0,
        };

        assert_eq!(state.game_result(), Some(Winner(X)));
        assert!(state.is_terminal());

        let state = TicTacToeState {
            board: [[x, x, None], [o, None, None], [x, None, None]],
            turn: 0,
        };

        assert_eq!(state.game_result(), None);
        assert!(!state.is_terminal());

        //X  X  O
        //X  X  O
        //.  O  .
        let state = TicTacToeState {
            board: [[x, x, o], [x, x, o], [None, o, None]],
            turn: 0,
        };

        assert_eq!(state.game_result(), None);
        assert!(!state.is_terminal());

        //X  X  O
        //X  X  O
        //O  O  O

        let state = TicTacToeState {
            board: [[x, x, o], [x, x, o], [o, o, o]],
            turn: 0,
        };

        assert_eq!(state.game_result(), Some(Winner(O)));
        assert!(state.is_terminal());
    }

    #[test]
    fn test_evals() {
        let evaluator = TicTacToeEvaluator;
        let eval = |state: &TicTacToeState<3>| -> f32 { evaluator.evaluate(state).unwrap() };

        let state = TicTacToeState {
            board: [[None, None, None], [None, None, None], [None, None, None]],
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
    fn perft() {
        type TTT = TicTacToe<3>;

        let state = TTT::initial_state();
        let rs = perft::perft::<TTT>(&state, 9, 5);
        assert_eq!(rs.nodes(), 255_168);

        println!(
            "TicTacToe<3> Perft(9) finished in {:.2}s -- Throughput: {}",
            rs.time().as_secs_f64(),
            rs.nps()
        );
    }
}
