use std::fmt::{Display, Formatter};
use std::ops::Index;
use glasswing::agents::{Evaluator, SymmetricEvaluation};
use glasswing::core::{Game, GameResult, GwAction, GwState, Team};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NTicTacToe<const N: usize>;

impl<const N: usize> Game for NTicTacToe<N> {
    type State = NTTTState<N>;
    type Action = NTTTAction;
    type Team = Team;
    type GameResult = GameResult<Self>;
    type EvalType = i32;

    fn initial_state() -> Self::State {
        NTTTState {
            board: [[None; N]; N],
            player: Team::One,
        }
    }

    fn starting_team() -> Self::Team {
        Team::One
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NTTTAction {
    row: usize,
    col: usize,
}

impl NTTTAction {
    pub fn new(row: usize, col: usize) -> Self {
        Self {
            row,
            col,
        }
    }
}

impl Display for NTTTAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Action {{ row: {}, col: {} }}", self.row, self.col)
    }
}

impl<const N: usize> GwAction<NTicTacToe<N>> for NTTTAction {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NTTTState<const N: usize> {
    board: [[Option<Team>; N]; N],
    player: Team,
}

impl<const N: usize> Index<usize> for NTTTState<N> {
    type Output = [Option<Team>; N];

    fn index(&self, index: usize) -> &Self::Output {
        &self.board[index]
    }
}
impl<const N: usize> NTTTState<N> {
    pub fn from_board(board: [[Option<Team>; N]; N], cur_player: Team) -> Self {
        Self {
            board,
            player: cur_player,
        }
    }

    pub fn is_full(&self) -> bool {
        for row in self.board.iter() {
            for cell in row.iter() {
                if cell.is_none() {
                    return false;
                }
            }
        }
        true
    }
}

impl<const N: usize> Display for NTTTState<N>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.board.iter() {
            for cell in row.iter() {
                match cell {
                    Some(Team::One) => write!(f, "X")?,
                    Some(Team::Two) => write!(f, "O")?,
                    None => write!(f, "·")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<const N: usize> GwState<NTicTacToe<N>> for NTTTState<N> {
    type ActionIter = NTTTActionIter<N>;

    fn actions(&self) -> Self::ActionIter {
        NTTTActionIter::new(self.clone())
    }

    fn team_to_move(&self) -> Team {
        self.player
    }

    // Assume that the action is legal in this state.
    // therefore, the state is not terminal.
    fn apply_action(&self, action: &NTTTAction) -> Self {
        let mut new_board = self.board.clone();
        new_board[action.row][action.col] = Some(self.team_to_move());
        Self {
            board: new_board,
            player: self.team_to_move().opponent(),
        }
    }

    fn is_terminal(&self) -> bool {
        self.game_result().is_some()
    }

    fn game_result(&self) -> Option<GameResult<NTicTacToe<N>>> {
        for &row in self.board.iter() {
            if row[0].is_some() && row.iter().all(|&cell| cell == row[0]) {
                return Some(GameResult::Win(row[0].unwrap()));
            }
        }

        for col_idx in 0..N {
            if self.board[0][col_idx].is_some() && (0..N).all(|i| self.board[i][col_idx] == self.board[0][col_idx]) {
                return Some(GameResult::Win(self.board[0][col_idx].unwrap()));
            }
        }

        if self.board[0][0].is_some() && (0..N).all(|i| self.board[i][i] == self.board[0][0]) {
            return Some(GameResult::Win(self.board[0][0].unwrap()));
        }

        if self.board[0][N - 1].is_some() && (0..N).all(|i| self.board[i][N - 1 - i] == self.board[0][N - 1]) {
            return Some(GameResult::Win(self.board[0][N - 1].unwrap()));
        }

        if self.is_full() {
            return Some(GameResult::Draw);
        }

        None // game not over yet
    }
}

pub struct NTTTActionIter<const N: usize> {
    state: NTTTState<N>,
    is_terminal: bool,
    row: usize,
    col: usize,
}

impl<const N: usize> NTTTActionIter<N> {
    pub fn new(state: NTTTState<N>) -> Self {
        let is_terminal = state.is_terminal();

        Self {
            state,
            is_terminal,
            row: 0,
            col: 0,
        }
    }
}

impl<const N: usize> Iterator for NTTTActionIter<N> {
    type Item = NTTTAction;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.is_terminal {
            if self.row < N {
                if self.state.board[self.row][self.col].is_none() {
                    let result = Some(NTTTAction::new(self.row, self.col));

                    self.col += 1;
                    if self.col == N {
                        self.row += 1;
                        self.col = 0;
                    }

                    return result;
                }
            } else {
                self.is_terminal = true;
                return None;
            }

            self.col += 1;
            if self.col == N {
                self.row += 1;
                self.col = 0;
            }
        }
        None
    }
}

pub struct NTTTEvaluator;

impl<const N: usize> Evaluator<NTicTacToe<N>> for NTTTEvaluator {
    fn evaluate_for(&mut self, state: &NTTTState<N>, team: &Team) -> i32 {
        match state.game_result() {
            Some(result) => {
                match result {
                    GameResult::Win(winner) => {
                        if winner == *team {
                            100
                        } else {
                            -100
                        }
                    }
                    GameResult::Draw => {
                        0
                    }
                }
            }
            None => {
                0
            }
        }
    }
}


// Yes, our evaluator is symmetric.
impl<const N: usize> SymmetricEvaluation<NTicTacToe<N>> for NTTTEvaluator {}