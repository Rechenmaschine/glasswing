use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub trait Agent {
    type Game: Game;

    fn identifier() -> String {
        String::from("agent")
    }

    /// Returns the recommended move for the given state
    fn recommend_move(
        &mut self,
        state: &<<Self as Agent>::Game as Game>::State,
    ) -> <<Self as Agent>::Game as Game>::Action;
}

pub trait Evaluator<G: Game> {
    fn identifier() -> String {
        String::from("evaluator")
    }
    /// Evaluates the given state for a two-player game.
    ///
    /// The magnitude of the score indicates how likely a player is to win.
    /// - A positive score indicates that the first player is winning
    /// - A negative score indicates that the second player is winning
    /// - A score of 0 indicates that the game is likely a draw
    fn evaluate(&self, state: &G::State) -> f32;
}

/// Trait for a game, that links together all necessary types for a game
pub trait Game: Sized + Serialize + DeserializeOwned {
    type State: State<Self>;
    type Action: Action<Self>;
    type Team: Team<Self>;

    type GameResult: GameResult<Self>;

    /// Returns the initial state of the game.
    /// This should be an invalid state, a starting position that is not reachable by any action.
    fn initial_state() -> Self::State;

    /// Returns the starting team of the game.
    /// This is the team that does the first ply, not necessarily the team in [Self::initial_state].
    fn starting_team() -> Self::Team;
}

pub trait GameResult<G: Game>: Clone + Debug + Serialize + DeserializeOwned {
    /// The winner of the game
    fn winner(&self) -> Option<G::Team>;

    /// The loser of the game
    fn loser(&self) -> Option<G::Team> {
        self.winner().map(|t| t.next())
    }

    /// Returns true, if the game is a draw
    fn is_draw(&self) -> bool;
}

/// This trait describes a state in a game. It contains all information necessary to play the game.
/// A game state can be in 4 different states:
/// - **Initial**: State with ply 0. This is an invalid state, that cannot be reached by any action.
/// This should be used for setup, representing the starting position of the game.
/// - **Await**: Reached by incrementing the ply. This is the beginning of a turn, where the current player can choose an action.
/// - **Applied**: Reached by applying an action. This is the state, in which an action has been applied. This effectively ends the turn of the current player.
/// - **Terminal**: Reached by applying an action from "applied". This is a state, where the game is over and a winner can be determined.
///
/// Certain actions, such as incrementing the ply or applying an action, are only allowed in certain states.
/// To avoid invariants, the user should refrain from calling functions in the "await" state, by calling [Self::next_state],
/// which increments the ply and applies the action.
pub trait State<G: Game<State = Self>>:
    Clone + Debug + Serialize + for<'a> Deserialize<'a>
{
    /// Returns a vector of all possible actions that can be taken from this state
    fn actions(&self) -> Vec<G::Action>;

    /// Returns a vector of all possible substates that can be reached from this state
    fn substates(&self) -> Vec<Self> {
        self.actions().iter().map(|a| self.next_state(a)).collect()
    }

    /// Returns the team whose turn it is to play in the current state.
    ///
    /// This implementation should by consistent with [Team::in_ply]
    fn current_team(&self) -> G::Team {
        Team::in_ply(self.ply())
    }

    /// Returns the current ply of the game. A ply is a half move, ie. the action of one player.
    /// The initial state has ply 0.
    fn ply(&self) -> usize;

    /// Returns the next state after applying the given action and advancing the ply
    ///
    /// **Note: You probably do not mean to reimplement this function**
    fn next_state(&self, action: &G::Action) -> Self {
        self.advance_ply().apply_action(action)
    }

    /// Applies an action to this state and returns the resulting state
    ///
    /// This function **should not** advance the state to the next ply. For this purpose, implement [Self::advance_ply] instead.
    fn apply_action(&self, action: &G::Action) -> Self;

    /// Advances the game state by one ply, ending the turn of the current player
    fn advance_ply(&self) -> Self;

    /// Returns true, if this state is terminal, ie. if the game is over
    fn is_terminal(&self) -> bool;

    /// Returns the game result or None, if the game is not over
    fn game_result(&self) -> Option<G::GameResult>;
}

pub trait Team<G: Game<Team = Self>>:
    Copy + Clone + Debug + Eq + PartialEq + Serialize + DeserializeOwned
{
    /// In the total order of teams, return the team after this one
    fn next(&self) -> Self;

    /// Returns the nth team that plays next
    fn nth(&self, n: isize) -> Self {
        if n % 2 == 0 {
            *self
        } else {
            self.next()
        }
    }

    /// Returns the team that plays the current ply.
    fn in_ply(ply: usize) -> Self {
        G::starting_team().nth(ply as isize - 1)
    }
}

pub trait Action<G: Game<Action = Self>>: Clone + Debug + Serialize + DeserializeOwned {}
