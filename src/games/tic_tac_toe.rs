use std::fmt;
use std::fmt::Formatter;
use crate::core::{Action, Game, GameResult, State, Team};

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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct TicTacToeState<const N: usize> {
    board: [[Option<TicTacToeTeam>; N]; N],
    turn: usize,
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

    fn ply(&self) -> usize {
        self.turn
    }

    fn apply_action(&self, action: &TicTacToeAction) -> Self {
        let mut new_board = self.board.clone();
        new_board[action.col][action.row] = Some(self.current_team());
        Self {
            board: new_board,
            turn: self.turn,
        }
    }

    fn advance_ply(&self) -> Self {
        Self {
            board: self.board,
            turn: self.turn + 1,
        }
    }

    fn is_terminal(&self) -> bool {
        // check if all tiles are filled
        for row in self.board.iter() {
            for tile in row.iter() {
                if tile.is_none() {
                    return false;
                }
            }
        }
        true
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

        if self.is_terminal() {
            Some(TicTacToeResult::Draw)
        } else {
            None
        }
    }
}

impl<const N: usize> fmt::Display for TicTacToeState<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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

#[cfg(test)]
mod tests {
    #![allow(unused)]

    use std::time::Duration;
    use log::{info, debug, error, warn, trace};
    use pretty_env_logger::env_logger::builder;
    use rand::rngs::OsRng;
    use crate::agents::random_agent::RandomAgent;
    use crate::agents::simple_agent::SimpleAgent;
    use crate::core::Match;
    use super::*;

    #[test]
    fn test() {
        type TTT = TicTacToe<4>;

        // init logger
        builder().filter_level(log::LevelFilter::Info).init();

        let mut wins_a = 0;
        let mut wins_b = 0;
        let mut draws = 0;

        for i in 0..2000 {
            let rng1: RandomAgent<TTT, _> = RandomAgent::new(OsRng::default());
            let rng2: RandomAgent<TTT, _> = RandomAgent::new(OsRng::default());
            let simp1: SimpleAgent<TTT> = SimpleAgent::new();
            let simp2: SimpleAgent<TTT> = SimpleAgent::new();

            let mut pit: Match<TTT> = Match::new(rng2, rng1)
                .with_time_limit(Duration::ZERO)
                .enforce_time_limit(false)
                .check_actions(true);

            // playout. If draw, print board
            let result = pit.playout();
            match result {
                Ok(res) => {
                    trace!("{}", res.state());
                    trace!("{:?}", res.game_result().unwrap());

                    match res.game_result().unwrap() {
                        TicTacToeResult::Winner(TicTacToeTeam::X) => wins_a += 1,
                        TicTacToeResult::Winner(TicTacToeTeam::O) => wins_b += 1,
                        TicTacToeResult::Draw => draws += 1,
                    }
                }
                Err(err) => {
                    error!("{:?}", err);
                }
            }
        }

        // print statistics
        info!("\n======= STATISTICS =======\n\tWins A: {}\n\tWins B: {}\n\tDraws: {}\n==========================", wins_a, wins_b, draws);
    }
}