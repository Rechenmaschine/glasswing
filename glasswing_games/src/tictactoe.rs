use std::fmt::{Display, Formatter};
use glasswing::agents::Evaluator;
use glasswing::core::Team::{One, Two};
use glasswing::core::{Game, GameResult, GwAction, GwState, GwTeam, Team};
use ordered_float::OrderedFloat;

pub struct TTTHeuristic;

impl Evaluator<TicTacToe> for TTTHeuristic {
    fn evaluate(&mut self, state: &TTTState) -> OrderedFloat<f32> {
        match state.game_result() {
            Some(GameResult::Win(team)) => {
                return OrderedFloat(
                    100.0 * <Team as GwTeam<TicTacToe>>::polarity(&team).sign() as f32,
                );
            }
            Some(GameResult::Draw) => OrderedFloat(0.0),
            None => OrderedFloat(0.0),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TTTAction {
    mask: u16,
}

impl Display for TTTAction{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "TTTAction {{ pos: {} }}", self.mask.checked_ilog2().expect("Mask must have a single bit set"))
    }
}

impl<G: Game<Action = TTTAction>> GwAction<G> for TTTAction {}

#[derive(Clone, Debug)]
pub struct TicTacToe;

impl Game for TicTacToe {
    type State = TTTState;
    type Action = TTTAction;
    type Team = Team;
    type GameResult = GameResult<TicTacToe>;
    type EvalType = OrderedFloat<f32>;

    fn initial_state() -> Self::State {
        TTTState {
            crosses: 0,
            noughts: 0,
            player: One,
            is_terminal: false,
        }
    }

    fn starting_team() -> Self::Team {
        One
    }
}

#[derive(Clone, Debug)]
pub struct TTTState {
    crosses: u16,
    noughts: u16,
    player: Team,
    is_terminal: bool,
}

impl GwState<TicTacToe> for TTTState {
    type ActionIter = TTTActionIter;

    #[inline]
    fn actions(&self) -> Self::ActionIter {
        TTTActionIter::new(self.clone())
    }

    #[inline]
    fn count_actions(&self) -> usize {
        (!self.is_terminal as usize) * (9 - (self.crosses | self.noughts).count_ones() as usize)
    }

    #[inline]
    fn team_to_move(&self) -> Team {
        self.player
    }

    #[inline]
    #[must_use]
    fn apply_action(&self, action: &TTTAction) -> Self {
        match self.player {
            One => {
                // check win conditions
                let crosses = self.crosses | action.mask;
                let ended = (crosses | self.noughts) == 0b111111111;
                Self {
                    crosses,
                    noughts: self.noughts,
                    player: Two,
                    is_terminal: win_condition(crosses) || ended,
                }
            }
            Two => {
                // check win conditions
                let noughts = self.noughts | action.mask;
                let ended = (self.crosses | noughts) == 0b111111111;
                Self {
                    crosses: self.crosses,
                    noughts,
                    player: One,
                    is_terminal: win_condition(noughts) || ended,
                }
            }
        }
    }

    #[inline]
    fn is_terminal(&self) -> bool {
        self.is_terminal
    }

    #[inline]
    fn game_result(&self) -> Option<GameResult<TicTacToe>> {
        if win_condition(self.crosses) {
            Some(GameResult::Win(One))
        } else if win_condition(self.noughts) {
            Some(GameResult::Win(Two))
        } else if self.crosses | self.noughts == 0b111111111 {
            Some(GameResult::Draw)
        } else {
            None
        }
    }
}
impl std::fmt::Display for TTTState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut board_str = String::new();

        for i in 0..9 {
            let mask = 1 << i;
            board_str.push_str(if self.crosses & mask != 0 {
                "X"
            } else if self.noughts & mask != 0 {
                "O"
            } else {
                "·"
            });

            if i % 3 == 2 {
                board_str.push_str("\n");
            } else {
                board_str.push_str(" ");
            }
        }

        write!(f, "{}", board_str)
    }
}

pub struct TTTActionIter {
    fields: u16,
}

impl TTTActionIter {
    #[inline]
    fn new(state: TTTState) -> Self {
        let not_set = !(state.crosses | state.noughts);
        let fields = (!state.is_terminal) as u16 * (0b111111111 & not_set);
        Self { fields }
    }
}

impl Iterator for TTTActionIter {
    type Item = TTTAction;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        return if self.fields == 0 {
            None
        } else {
            let mask = self.fields & !(self.fields - 1);
            self.fields ^= mask;
            Some(TTTAction { mask })
        };
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let count = self.fields.count_ones() as usize;
        (count, Some(count))
    }
}

#[cfg(feature = "simd_support")]
#[inline]
fn win_condition(mask: u16) -> bool {
    use std::simd::{u16x8, SimdPartialEq};

    let mask_vector = u16x8::splat(mask);
    let win_masks = u16x8::from([
        0b000000111,
        0b000111000,
        0b111000000,
        0b001001001,
        0b010010010,
        0b100100100,
        0b100010001,
        0b001010100,
    ]);
    (mask_vector & win_masks).simd_eq(win_masks).any()
}

#[cfg(not(feature = "simd_support"))]
#[inline]
fn win_condition(mask: u16) -> bool {
    let win_masks = [
        0b000000111,
        0b000111000,
        0b111000000,
        0b001001001,
        0b010010010,
        0b100100100,
        0b100010001,
        0b001010100,
    ];
    win_masks
        .iter()
        .any(|&win_mask| mask & win_mask == win_mask)
}