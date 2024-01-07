use glasswing::agents::Evaluator;
use glasswing::core::{Game, GameResult, GwState, Team};
use std::fmt::Display;
use std::ops::Index;

#[derive(Debug, Clone)]
pub struct Connect4;

impl Game for Connect4 {
    type State = C4State;
    type Action = C4Action;
    type Team = Team;
    type GameResult = GameResult<Self::Team>;
    type EvalType = i32;

    fn initial_state() -> Self::State {
        C4State {
            board: [Column {
                one: 0,
                two: 0,
                height: 0,
            }; 7],
            player: Team::One,
            game_result: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct C4Action {
    column: u8,
}

impl Display for C4Action {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Action {{ column: {} }}", self.column)
    }
}

#[allow(dead_code)]
impl C4Action {
    pub fn new(column: u8) -> Self {
        Self { column }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Column {
    one: u8,
    // LSB is lowest tile
    two: u8,
    // teams one and two have their tiles
    height: u8, // height of column (max 6)
}

#[derive(PartialEq)]
pub enum Tile {
    Empty,
    Colour(Team),
}

impl Index<usize> for Column {
    type Output = Tile;

    fn index(&self, index: usize) -> &Self::Output {
        let mask = 1 << index;
        if self.one & mask != 0 {
            &Tile::Colour(Team::One)
        } else if self.two & mask != 0 {
            &Tile::Colour(Team::Two)
        } else {
            &Tile::Empty
        }
    }
}

#[derive(Clone, Debug)]
pub struct C4State {
    board: [Column; 7],
    player: Team,
    game_result: Option<GameResult<Team>>,
}

impl GwState<Connect4> for C4State {
    type ActionIter = C4ActionIter;

    #[inline]
    fn actions(&self) -> Self::ActionIter {
        C4ActionIter::new(self.clone())
    }

    #[inline]
    fn team_to_move(&self) -> Team {
        self.player
    }

    /// assume that the action is valid, therefore this is not a terminal game state.
    #[inline]
    #[must_use]
    fn apply_action(&self, action: &C4Action) -> Self {
        assert!(!self.is_terminal()); // applying an action to a terminal state is undefined.

        //// Step 1: generate new board ////
        let idx = action.column as usize;
        let mut new_board = self.board.clone();

        let new_col = &mut new_board[idx];
        let old_height = new_col.height;

        match self.team_to_move() {
            Team::One => new_col.one |= 1 << old_height,
            Team::Two => new_col.two |= 1 << old_height,
        }
        new_col.height += 1;

        let proxy = match self.team_to_move() {
            Team::One => [
                new_board[0].one,
                new_board[1].one,
                new_board[2].one,
                new_board[3].one,
                new_board[4].one,
                new_board[5].one,
                new_board[6].one,
            ],
            Team::Two => [
                new_board[0].two,
                new_board[1].two,
                new_board[2].two,
                new_board[3].two,
                new_board[4].two,
                new_board[5].two,
                new_board[6].two,
            ],
        };

        //// Step 2: check for win ////

        let bits = proxy[idx];
        let new_height = new_board[idx].height;

        // check for vertical win
        if new_height >= 4 {
            let mask = 0b1111;

            if ((bits & mask) == mask)
                || ((bits & mask << 1) == mask << 1)
                || ((bits & mask << 2) == mask << 2)
            {
                return C4State {
                    board: new_board,
                    player: self.team_to_move().opponent(),
                    game_result: Some(GameResult::Win(self.player)),
                };
            }
        }

        // check for horizontal win
        let mut bits = 0;
        for i in 0..7 {
            bits |= (proxy[i] >> old_height) & 0x1;
            bits <<= 1;
        }

        // check whether mask matches
        let mut mask = 0b11110;
        for _ in 0..4 {
            if (bits & mask) == mask {
                return C4State {
                    board: new_board,
                    player: self.team_to_move().opponent(),
                    game_result: Some(GameResult::Win(self.player)),
                };
            }
            mask <<= 1;
        }

        // check for diagonal win

        if check_diagonals(&proxy, idx as u8, old_height) {
            return C4State {
                board: new_board,
                player: self.team_to_move().opponent(),
                game_result: Some(GameResult::Win(self.player)),
            };
        }

        //// Step 3: check for draw ////
        // check if all columns are full
        if new_board.iter().all(|col| col.height >= 6) {
            return C4State {
                board: new_board,
                player: self.team_to_move().opponent(),
                game_result: Some(GameResult::Draw),
            };
        }

        // else state not terminal.
        Self {
            board: new_board,
            player: self.team_to_move().opponent(),
            game_result: None,
        }
    }

    #[inline]
    fn is_terminal(&self) -> bool {
        self.game_result.is_some()
    }

    #[inline]
    fn game_result(&self) -> Option<GameResult<Team>> {
        self.game_result.clone()
    }
}

impl C4State {
    pub fn from_pretty(pretty: &str, game_result: Option<GameResult<Team>>) -> Self {
        let mut new_state = Self {
            board: [Column {
                one: 0,
                two: 0,
                height: 0,
            }; 7],
            player: Team::One,
            game_result,
        };

        let pretty = pretty.replace(" ", "").replace("\n", "");
        for (i, c) in pretty.chars().enumerate() {
            let col = i % 7;
            let row = 5 - (i / 7);
            let tile = match c {
                '🔘' => Tile::Empty,
                '🔴' => Tile::Colour(Team::One),
                '🟡' => Tile::Colour(Team::Two),
                x => panic!("Invalid character in pretty string for Connect4: {}", x),
            };
            if tile != Tile::Empty {
                new_state.player = new_state.player.opponent();
            }
            new_state.set_tile(col, row, tile);
        }

        // calculate heights
        for col in new_state.board.iter_mut() {
            let mut height = 0;
            for i in 0..6 {
                if col[i] != Tile::Empty {
                    height += 1;
                }
            }
            col.height = height;
        }

        new_state
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: Tile) {
        let col = &mut self.board[x];
        match tile {
            Tile::Empty => {
                let mask = !(1 << y);
                col.one &= mask;
                col.two &= mask;
            }
            Tile::Colour(Team::One) => {
                let mask = 1 << y;
                col.one |= mask;
                col.two &= !mask;
            }
            Tile::Colour(Team::Two) => {
                let mask = 1 << y;
                col.one &= !mask;
                col.two |= mask;
            }
        }
    }
}

#[inline]
pub fn check_diagonals(board: &[u8; 7], x: u8, y: u8) -> bool {
    let board = [
        0, board[6], board[5], board[4], board[3], board[2], board[1], board[0],
    ];
    let board = u64::from_be_bytes(board);
    return check_up_diagonal(board, x, y) || check_down_diagonal(board, x, y);
}

#[inline]
pub fn check_up_diagonal(board: u64, x: u8, y: u8) -> bool {
    let bitboard = gen_up_diagonal(x, y);
    let new = (bitboard & board).to_le_bytes();

    let mut count = 0;
    for &x in new.iter().take(7) {
        if count >= 4 {
            return true;
        }
        if x != 0 {
            count += 1;
        } else {
            count = 0;
        }
    }
    return count >= 4;
}

#[inline]
pub fn check_down_diagonal(board: u64, x: u8, y: u8) -> bool {
    let bitboard = gen_down_diagonal(x, y);
    let new = (bitboard & board).to_le_bytes();

    let mut count = 0;
    for &x in new.iter().take(7) {
        if count >= 4 {
            return true;
        }
        if x != 0 {
            count += 1;
        } else {
            count = 0;
        }
    }
    return count >= 4;
}

const DP: u64 = 0x8040201008040201;
const DS: u64 = 0x0102040810204080;

#[inline]
const fn gen_up_diagonal(x: u8, y: u8) -> u64 {
    let mut m = 0;
    if x >= y {
        m |= DP << ((x - y) << 3);
    } else {
        m |= DP >> ((y - x) << 3);
    }
    m
}

#[inline]
const fn gen_down_diagonal(x: u8, y: u8) -> u64 {
    let mut m = 0;
    let z = x;
    let y = 7 - y;
    if z >= y {
        m |= DS << ((z - y) << 3);
    } else {
        m |= DS >> ((y - z) << 3);
    }
    m
}

impl Display for C4State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut board_str = String::new();

        for i in 0..6 {
            for j in 0..7 {
                let tile = &self.board[j][5 - i];
                board_str.push_str(match tile {
                    Tile::Empty => "🔘",
                    Tile::Colour(Team::One) => "🔴",
                    Tile::Colour(Team::Two) => "🟡",
                });
            }
            board_str.push('\n');
        }
        board_str.pop(); // remove trailing newline

        write!(f, "{}", board_str)
    }
}

pub struct C4ActionIter {
    idx: usize,
    // col idx
    heights: [u8; 7],
}

impl C4ActionIter {
    #[inline]
    fn new(state: C4State) -> Self {
        if state.is_terminal() {
            return Self {
                idx: 7,
                heights: [0; 7],
            };
        }
        let heights = [
            state.board[0].height,
            state.board[1].height,
            state.board[2].height,
            state.board[3].height,
            state.board[4].height,
            state.board[5].height,
            state.board[6].height,
        ];
        Self { idx: 0, heights }
    }
}

impl Iterator for C4ActionIter {
    type Item = C4Action;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while self.idx < 7 {
            if self.heights[self.idx] >= 6 {
                self.idx += 1;
                continue;
            } else {
                let action = C4Action {
                    column: self.idx as u8,
                };
                self.idx += 1;
                return Some(action);
            }
        }
        return None;
    }
}

pub struct C4Heuristic;

impl Evaluator<Connect4> for C4Heuristic {
    #[inline]
    fn evaluate_for(&mut self, state: &C4State, team: &Team) -> i32 {
        return match state.game_result {
            Some(ref x) => match x {
                GameResult::Win(winner) => {
                    let turn = state.board.iter().map(|x| x.height).sum::<u8>() as i32;
                    if *winner == *team {
                        1000 - turn
                    } else {
                        -1000 + turn
                    }
                }
                GameResult::Draw => 0,
            },
            None => 0,
        };
    }
}
