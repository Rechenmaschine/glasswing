use crate::core::Error;
use std::fmt::Debug;
use std::time::Duration;

/// `StateStage` describes the stages of a state in terms of a the state machine.
#[allow(dead_code)]
enum StateStage {
    /// This stage describes the state in ply 0. This is an invalid state, that cannot be reached
    /// by any action. This should be used for setup, representing the starting position of
    /// the game. In the state machine, the `Initial` stage is the starting state. Incrementing
    /// the turn counter will move the state from `Initial` to `Await`.
    Initial,

    /// In the state machine, the `Await` stage describes the state in which the `turn` has
    /// been incremented, but no action has been applied yet. This stage marks the beginning
    /// of a turn. In this stage, an [crate::core::Agent] is asked to recommend an action for
    /// the state. In the state machine, applying an action will move the state from `Await`
    Await,

    /// In the `Applied` stage, an action has been applied to the state. This stage marks the
    /// end of a turn. In this stage, the state can be evaluated, checked for terminal, and
    /// the next player can be determined. In the state machine, incrementing the turn counter
    /// will move the state from `Await` to `Applied`. If the game is over instead, the state
    /// will move to `Terminal`.
    Applied,

    /// In the `Terminal` stage, the game has ended. The state can be evaluated and the
    /// `GameResult` can be determined. In the state machine, this stage is final.
    Terminal,
}

// The `MaybeDeserializeOwned` and `MaybeSerialize` traits provide conditional
// serde support for serializing and deserializing objects. If the feature "serde_support" is
// enabled, they refer to serde's `DeserializeOwned` and `Serialize` traits, otherwise they refer
// to [std::any::Any].
#[cfg(feature = "serde_support")]
pub use serde::de::DeserializeOwned as MaybeDeserializeOwned;
#[cfg(feature = "serde_support")]
pub use serde::ser::Serialize as MaybeSerialize;

#[cfg(not(feature = "serde_support"))]
pub use std::any::Any as MaybeSerialize;
#[cfg(not(feature = "serde_support"))]
pub use std::any::Any as MaybeDeserializeOwned;

/// The `Agent` trait represents an entity that can interact with and make decisions in a game.
///
/// This could be a human player, an AI, a network player, etc. The agent is responsible for deciding which actions to take
/// at each state of the game, given a certain time limit.
///
/// In another sense, the agent can also be used as a `bridge`, ie. act as an adapter between a game
/// contest and a game agent, forwarding calls to the agent and handling any errors that arise, also managing
/// threading and time limits on the agent's side. This trait can be used for implementing features like
/// pondering and logging in games or even connecting an agent to a server.
///
/// # Integrity
///
/// Implementations of this trait must not cause panics and should handle possible failures gracefully
/// via the [Error] type.
///
/// # Usage
///
/// This trait is used to implement entities that can make decisions in the game.
///
/// For example, in a chess AI, an implementation of this trait could use a search algorithm to choose the best move:
///
/// ```rust
/// pub struct ChessAI;
///
/// impl Agent for ChessAI {
///     type Game = Chess;
///
///     fn recommend_action(
///         &mut self,
///         state: &<<Self as Agent>::Game as Game>::State,
///         time_limit: Duration,
///     ) -> Result<<<Self as Agent>::Game as Game>::Action, Error> {
///         // Use a search algorithm to choose the best move
///     }
/// }
/// ```
pub trait Agent {
    type Game: Game;

    /// Returns the recommended action for the given state.
    ///
    /// # Arguments
    ///
    /// * `state` - The state for which an action should be recommended. It is provided in the
    /// [StateStage::Await] stage.
    /// * `time_limit` - The time limit for the agent to complete recommendation of an action.
    fn recommend_action(
        &mut self,
        state: &<<Self as Agent>::Game as Game>::State,
        time_limit: Duration,
    ) -> Result<<<Self as Agent>::Game as Game>::Action, Error>;
}

/// The `Evaluator` trait defines a heuristic function for evaluating the state of a game.
///
/// This trait is commonly used in game theory and AI algorithms like minimax and alpha-beta pruning
/// to approximate the value of a game state without exploring all of its potential outcomes.
///
/// # Integrity
///
/// The implementation of `evaluate` should be safe in all cases and shouldn't panic or cause
/// any side effects.
///
/// The returned value should respect the contract: positive for advantage of the first player,
/// negative for advantage of the second player, and zero for a balanced game.
///
/// # Usage
///
/// This trait is typically implemented for a struct that encapsulates a strategy
/// or heuristic for a particular game. The `evaluate` function is then used in game-playing
/// algorithms to estimate the advantage of a player in a certain state.
///
/// For example, in a chess game an evaluator might count the total value of each player's remaining pieces:
///
/// ```rust
/// pub struct ChessEvaluator;
///
/// impl Evaluator<Chess> for ChessEvaluator {
///     fn evaluate(&self, state: &Chess::State) -> Result<f32, Error> {
///         // implementation that counts the pieces and returns a score
///     }
/// }
/// ```
pub trait Evaluator<G: Game> {
    /// Evaluates the given state for a two-player game.
    ///
    /// The magnitude of the score indicates how likely a player is to win.
    /// - A positive score indicates that the first player is winning
    /// - A negative score indicates that the second player is winning
    /// - A score of 0 indicates that the game is likely a draw
    ///
    /// # Arguments
    ///
    /// * `state` - The state to evaluate. This can be called in any stage of the state machine,
    /// depending on the algorithm that uses it.
    fn evaluate(&self, state: &G::State) -> Result<f32, Error>;
}

/// The `Game` trait encapsulates the concept of a game in this framework.
///
/// It links together all necessary types for a game like the state of the game,
/// the available actions at each state, the teams or players in the game, and the possible results of the game.
///
/// # Integrity
///
/// Implementations of this trait must adhere to the rules and logic of the represented game.
/// In particular, the implementation of the `initial_state` and `starting_team` methods should provide valid,
/// game-specific initial conditions. Likewise, the associated types must respect the rules and state transitions of the game.
///
/// # Usage
///
/// This trait is usually implemented for a struct representing a particular game.
/// The implementation defines the rules, states, and entities (like teams or players) of the game.
///
/// For example, a simple implementation for a game of tic-tac-toe could look like this:
///
/// ```rust
/// pub struct Chess;
///
/// impl Game for Chess {
///     type State = ChessState;
///     type Action = ChessMove;
///     type Team = ChessTeam;
///     type GameResult = ChessResult;
///
///     fn initial_state() -> Self::State { /* ... */ }
///     fn starting_team() -> Self::Team { /* ... */ }
/// }
/// ```
///
/// Note: `MaybeSerialize` and `MaybeDeserializeOwned` are used for serialization and deserialization support.
pub trait Game: Sized + MaybeSerialize + MaybeDeserializeOwned {
    /// The type representing the state of the game. This could include the positions of
    /// all pieces in a chess game, the value of all cards in a card game, etc.
    type State: State<Self>;

    /// The type representing an action or move in the game. This could be a chess
    /// piece move, a card played, etc.
    type Action: Action<Self>;

    /// The type representing a team or a player in the game.
    type Team: Team<Self>;

    /// The type representing the result of a game. It usually includes information about
    /// the winner, loser, or whether the game was a draw.
    type GameResult: GameResult<Self>;

    /// Returns the initial state of the game. The initial state always has ply 0.
    /// This should be an invalid state, a starting position that is not reachable by any action.
    /// The initial state marks the starting point of the state machine.
    fn initial_state() -> Self::State;

    /// Returns the starting team of the game.
    /// This is the team that does the first ply, not necessarily the team in [Self::initial_state].
    fn starting_team() -> Self::Team;
}

/// The `GameResult` trait represents the outcome of a game.
///
/// It provides methods to query the winner, the loser, and whether the game resulted in a draw.
/// This trait is typically used in terminal states of a game, where the game's outcome can be determined.
///
/// # Usage
///
/// This trait is used as an associated type in the `Game` trait, and is returned from the `State`
/// trait's `game_result` method. It encapsulates the result of a game, providing information about
/// the winner, loser, and whether the game resulted in a draw.
///
/// Here is a basic example for a Chess game:
///
/// ```rust
///
/// pub struct ChessResult {
///     winner: Option<ChessTeam>,
///     ...
/// }
///
/// impl GameResult<Chess> for ChessResult {
///     fn winner(&self) -> Option<ChessTeam> {
///         self.winner
///     }
///
///     fn is_draw(&self) -> bool {
///         self.winner.is_none()
///     }
/// }
/// ```
///
/// Note: `MaybeSerialize` and `MaybeDeserializeOwned` are used for serialization and deserialization support.
pub trait GameResult<G: Game>: Clone + Debug + MaybeSerialize + MaybeDeserializeOwned {
    /// The winner of the game
    fn winner(&self) -> Option<G::Team>;

    /// The loser of the game
    fn loser(&self) -> Option<G::Team> {
        self.winner().map(|t| t.next())
    }

    /// Returns true, if the game is a draw
    fn is_draw(&self) -> bool;
}

/// This trait describes a state in a game.
///
/// A state in a game typically contains all the necessary information to reflect the current situation of the game.
/// Depending on the game, a state could include the positions of all pieces in a chess game or the current score in a sports game.
/// In this context, it's used to represent the different situations a game can be in and provides the necessary functionalities
/// to transition between these states and query relevant information about them.
///
/// It defines methods for retrieving the current player's team, all possible actions in the current state,
/// and the state resulting from taking an action, among others.
///
/// A `State` can be in one of the four different states listed in the [StateStage] enum.
///
/// To ensure game rule compliance, it's essential that the trait's methods are correctly implemented.
/// In particular, state transitions need to conform to the rules of the game, e.g., illegal moves should not be allowed.
///
/// # Integrity
///
/// Implementations of this trait should be mindful of potential game state integrity issues.
/// Actions should only be applied if they are valid within the current stage and state context.
///
/// # Usage
///
/// This trait is used as part of the `Game` trait's associated types.
///
/// For instance, you might implement `State` for a `Chess` struct like this:
///
/// ```rust
/// pub struct ChessState {
///     board: [[Option<Piece>; 8]; 8],
///     current_team: ChessTeam,
///     turn: u32,
///     ...
/// }
///
/// impl State<Chess> for ChessState {
///     ...
///     fn current_team(&self) -> ChessTeam {
///         self.current_team
///     }
///
///     fn actions(&self) -> Vec<ChessAction> {
///         ...
///     }
///     ...
/// }
/// ```
///
/// Note: `MaybeSerialize` and `MaybeDeserializeOwned` are used for serialization and deserialization support.
pub trait State<G: Game<State=Self>>:
Clone + Debug + MaybeSerialize + MaybeDeserializeOwned
{
    /// Returns true, if the provided action is legal in the current state
    /// By default, this function checks if the action is in the list of legal actions
    /// provided in [Self::actions]
    ///
    /// **State machine** - This function should only be called in the [StateStage::Await] stage.
    fn is_legal(&self, action: &G::Action) -> bool {
        self.actions().contains(action)
    }

    /// Returns a vector of all **legal** actions that can be taken from this state.
    ///
    /// **State machine** - This function should only be called in the [StateStage::Await] stage.
    fn actions(&self) -> Vec<G::Action>;

    /// Returns the team whose turn it is to play in the current state. This implementation
    /// should by consistent with [Team::in_ply]
    ///
    /// **State machine** - This function can be called in the any stage other than the [StateStage::Initial]
    /// stage, since the first step when beginning a new turn is to increment the turn counter.
    fn current_team(&self) -> G::Team {
        Team::in_ply(self.ply())
    }

    /// Returns the current ply of the game. The initial state should has a ply of 0. The ply
    /// is analog to the turn counter in a game. It is incremented at the *beginning* of a turn.
    ///
    /// **State machine** - This function can be called at any stage.
    fn ply(&self) -> usize;

    /// Returns the next state after applying the given action and incrementing the turn counter.
    ///
    /// # Arguments
    ///  - `action` - The action to apply to this state
    ///
    /// **State machine** - This function can be called in the [StateStage::Await] stage. The state
    /// returned should be in the [StateStage::Await] or [StateStage::Terminal] stage.
    ///
    /// **Note: This function should not be reimplemented**
    fn next_state(&self, action: &G::Action) -> Self {
        self.advance_ply().apply_action(action)
    }

    /// Applies an action to this state and returns the resulting state.
    ///
    /// This function **should not** advance the state to the next ply. For this purpose,
    /// use [Self::advance_ply] instead.
    ///
    /// **State machine** - This function can be called in the [StateStage::Await] stage.
    fn apply_action(&self, action: &G::Action) -> Self;

    /// Increments the turn counter, starting the turn of the next player.
    ///
    /// **State machine** - This function can be called in the [StateStage::Applied] stage.
    fn advance_ply(&self) -> Self;

    /// Returns whether this state is terminal, ie. if the game is over or not. This is analog
    /// to checking whether the state machine is in the [StateStage::Terminal] stage.
    ///
    /// **State machine** - This function can be called in the [StateStage::Applied] or
    /// [StateStage::Terminal] stage.
    fn is_terminal(&self) -> bool;

    /// Returns the game result or None, if the game is not over.
    ///
    /// **State machine** - This function should return `Some(game_result)` in the [StateStage::Terminal] stage
    /// and `None` in the [StateStage::Applied] stage.
    fn game_result(&self) -> Option<G::GameResult>;
}

/// This trait encapsulates the concept of a team or a player in a two-player game.
///
/// Implementations of this trait will differ based on the specifics of the game,
/// but common uses might include representing a color in a board game (like chess or checkers),
/// a specific player in a multiplayer game, or even the sole player in a single-player game.
///
/// The trait provides functionality for understanding team order, such as which team/player
/// comes next and which team/player is currently in play.
///
/// # Integrity
/// An assumption of the framework is that the the two teams always alternate.
/// Therefore, the `next` function will always return the other player/team.
///
/// # Usage
///
/// This trait is used as part of the `Game` trait's associated types.
/// It's used within the game mechanics to control the flow of game turns and determine the current player/team.
///
/// For instance, you might implement `Team` for a `Chess` struct like this:
///
/// ```rust
/// pub enum ChessTeam {
///     White,
///     Black,
/// }
///
/// impl Team<Chess> for ChessTeam {
///     fn next(&self) -> Self {
///         match self {
///             Self::White => Self::Black,
///             Self::Black => Self::White,
///         }
///     }
///     ...
/// }
/// ```
///
/// Note: `MaybeSerialize` and `MaybeDeserializeOwned` are used for serialization and deserialization support.
pub trait Team<G: Game<Team=Self>>:
Copy + Clone + Debug + Eq + PartialEq + MaybeSerialize + MaybeDeserializeOwned
{
    /// In the total order of teams, return the team after this one
    fn next(&self) -> Self;

    /// Returns the nth team that plays next
    /// If 0 is passed, then the current team is returned.
    fn nth(&self, n: isize) -> Self {
        if n % 2 == 0 {
            *self
        } else {
            self.next()
        }
    }

    /// Returns the team that plays the current ply. For example, if the current ply is 1,
    /// then the starting team plays. If the current ply is 2, then the team after that plays.
    fn in_ply(ply: usize) -> Self {
        G::starting_team().nth(ply as isize - 1)
    }
}

/// The `Action` trait encapsulates the concept of an action or a move in a game.
///
/// Implementations of this trait will represent a particular move a player or team can make, such
/// as moving a piece to a new position in chess, or playing a particular card in a card game.
///
/// # Integrity
///
/// An `Action` is not inherently legal in the context of a game. This is determined by the
/// `State` implementation. It is up to the user to ensure legality.
///
/// # Usage
///
/// In chess, you might represent a move as a struct with a start position and an end position:
///
/// ```rust
/// pub struct ChessMove {
///     from: (usize, usize),
///     to: (usize, usize),
/// }
///
/// impl Action<Chess> for ChessMove { /*...*/ }
/// ```
///
/// Note: `MaybeSerialize` and `MaybeDeserializeOwned` are used for serialization and deserialization support.
pub trait Action<G: Game<Action=Self>>:
Clone + Debug + PartialEq + MaybeSerialize + MaybeDeserializeOwned
{}
