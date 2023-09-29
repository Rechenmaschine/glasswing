use crate::core::serialization::{DeserializeAlias, SerializeAlias};
use anyhow::Error;
use std::fmt::Debug;
use std::hash::Hash;
use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MatchError<G: Game> {
    #[error("No legal actions available in state {0:?}")]
    NoAvailableActions(G::State),
    #[error("Invalid action in state (provided {action:?} in {state:?}")]
    IllegalAction { action: G::Action, state: G::State },
    #[error("Agent exceeded time limit: allowed {limit:?}, took {time:?}")]
    TimeLimitExceeded { limit: Duration, time: Duration },
}

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
///     fn select_action(
///         &mut self,
///         state: &<<Self as Agent>::Game as Game>::State,
///         time_limit: Duration,
///     ) -> Result<<<Self as Agent>::Game as Game>::Action, Error> {
///         // Use a search algorithm to choose the best move
///     }
/// }
/// ```
pub trait Agent<G: Game> {
    /// Returns the recommended action for the given state.
    ///
    /// # Arguments
    ///
    /// * `state` - The state for which an action should be recommended. It is provided in the
    /// [StateStage::Await] stage.
    /// * `time_limit` - The time limit for the agent to complete recommendation of an action.
    fn select_action(&mut self, state: &G::State, time_limit: Duration)
        -> Result<G::Action, Error>;
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
    /// Returns a value describing a state's desirability, **independent** from the team
    /// to move. This function may be called on terminal or non-terminal states.
    /// Therefore, it may be required to return a heuristic value for non-terminal states.
    ///
    /// For the evaluation function, the following should hold:
    ///
    /// - If `state.team_to_move().polarity() == Polarity::Positive` then evaluation
    /// should be *positive* if the state is *winning for the team to move*. Else, the
    /// evaluation should be negative.
    ///
    /// - If `state.team_to_move().polarity() == Polarity::Negative` then evaluation
    /// should be negative if the state is winning for the team to move. Else, the
    /// evaluation should be positive.
    ///
    ///
    /// This is **equivalent** to evaluating the state relative to the team to move
    /// and then multiplying the result by the polarity of the team to move, where
    /// positive indicates a winning state and negative indicates a losing state.
    ///
    /// Note that the magnitude of the evaluation may play a key role for the algorithm.
    /// For example, if polarity is positive, then minimax will prefer a state with
    /// a value of 10 over a state with a value of 1, even though both states are
    /// technically evaluated as winning by the evaluator.
    ///
    /// # Errors
    /// Errors should be handled gracefully using `anyhow::Error`, instead of panicking.
    fn evaluate(&self, state: &G::State) -> Result<f32, Error>;

    /// Returns the heuristic value of an action in a given state. This function may
    /// be called with any *legal* move of a *non-terminal* state. Calling the function
    /// with an illegal move or a terminal state is generally not well-defined.
    ///
    /// This function is used in algorithms such as alpha-beta pruning to order the
    /// moves of a state before visiting them, potentially increasing the number of
    /// pruned nodes.
    ///
    /// For the evaluation function, the following should hold. Let `post` be the
    /// resulting state after applying `action` to `state`.
    ///
    /// - If `post` is likely to be winning for the team to move, then the heuristic
    /// value should be *positive*. Else, the heuristic value should be *negative*.
    ///
    /// Note that the magnitude of the heuristic value may play a key role for
    /// the algorithm. For example, a state with a heuristic value of 10 will be
    /// preferred over a state with a heuristic value of 1, even though both states
    /// are technically considered good by the evaluator.
    ///
    /// # Errors
    /// This function **should generally not fail**. All errors are considered critical,
    /// therefore this function may panic if an error occurs.
    fn heuristic(&self, state: &G::State) -> f32;

    /// Evaluates an action in a given state. This function may be called with any
    /// *legal* move of a *non-terminal* state. Calling the function with an illegal
    /// move or a terminal state is generally not well-defined.
    ///
    /// This heuristic is **not** necessarily consistent with the heuristic function.
    /// While this is true for the default implementation, this function may be
    /// reimplemented to define a new heuristic and sacrifice accuracy for speed.
    ///
    /// For the evaluation function, the following should hold. Let `post` be the
    /// resulting state after applying `action` to `state`.
    ///
    /// - If `post` is likely to be winning for the team to move, then the heuristic
    /// value should be *positive*. Else, the heuristic value should be *negative*.
    ///
    /// Note that the magnitude of the heuristic value may play a key role for
    /// the algorithm. For example, an action with a heuristic value of 10 will be
    /// preferred over an action with a heuristic value of 1, even though both actions
    /// are technically considered good by the evaluator.
    ///
    /// # Errors
    /// This function **should generally not fail**. All errors are considered critical,
    /// therefore this function may panic if an error occurs.
    fn action_heuristic(&self, state: &G::State, action: &G::Action) -> f32 {
        let post = state.apply_action(action);
        self.heuristic(&post)
    }
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
/// Note: `SerializeAlias` and `DeserializeAlias` are used for serialization and deserialization support.
pub trait Game: Sized + Debug + SerializeAlias + DeserializeAlias + 'static {
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

    const NAME: &'static str;

    /// Return an instance of the game.
    fn new() -> Self;

    /// Executed before each turn, ie. before the action is applied
    fn before_turn(&mut self) {}

    /// Executed after each turn, ie. after the action has been applied
    fn after_turn(&mut self) {}

    fn current_state(&self) -> Self::State;

    fn apply_action(&mut self, action: &Self::Action);

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
/// Note: `SerializeAlias` and `DeserializeAlias` are used for serialization and deserialization support.
pub trait GameResult<G: Game>: Clone + Debug + SerializeAlias + DeserializeAlias {
    /// The winner of the game
    fn winner(&self) -> Option<G::Team>;

    /// Returns true, if the game is a draw
    fn is_draw(&self) -> bool;
}

#[derive(Debug, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub enum TwoPlayerGameResult<G: Game> {
    Winner(G::Team),
    Draw,
}

// Manual implementation of Clone because of the generic type parameter
impl<G: Game> Clone for TwoPlayerGameResult<G> {
    fn clone(&self) -> Self {
        match self {
            TwoPlayerGameResult::Winner(team) => TwoPlayerGameResult::Winner(*team),
            TwoPlayerGameResult::Draw => TwoPlayerGameResult::Draw,
        }
    }
}

impl<G: Game> GameResult<G> for TwoPlayerGameResult<G> {
    fn winner(&self) -> Option<G::Team> {
        match self {
            TwoPlayerGameResult::Winner(team) => Some(*team),
            TwoPlayerGameResult::Draw => None,
        }
    }

    fn is_draw(&self) -> bool {
        match self {
            TwoPlayerGameResult::Winner(_) => false,
            TwoPlayerGameResult::Draw => true,
        }
    }
}

/// The `State` trait describes a state in a game.
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
/// Note: `SerializeAlias` and `DeserializeAlias` are used for serialization and deserialization support.
// Send and Sync required for `anyhow::Error`
pub trait State<G: Game>: Clone + Debug + Send + Sync + SerializeAlias + DeserializeAlias {
    /// The type representing the actions that can be taken in this state.
    type ActionIterator: IntoIterator<Item = G::Action>;

    /// Returns true, if the provided action is legal in the current state
    /// By default, this function checks if the action is in the list of legal actions
    /// provided in [Self::actions]
    ///
    /// **State machine** - This function should only be called in the [StateStage::Await] stage.
    fn is_legal(&self, action: &G::Action) -> bool {
        self.actions().into_iter().any(|a| a == *action)
    }

    /// Returns all legal actions that can be taken from this state.
    ///
    /// **State machine** - This function should only be called in the [StateStage::Await] stage.
    fn actions(&self) -> Self::ActionIterator;

    /// Returns the number of legal actions that can be taken from this state.
    fn count_actions(&self) -> usize {
        self.actions().into_iter().count()
    }

    /// Returns the team whose turn it is to play in the current state. This implementation
    /// should by consistent with [Team::in_ply]
    ///
    /// **State machine** - This function can be called in the any stage other than the [StateStage::Initial]
    /// stage, since the first step when beginning a new turn is to increment the turn counter.
    fn team_to_move(&self) -> G::Team {
        Team::in_turn(self.turn())
    }

    /// Returns the current ply of the game. The initial state should has a ply of 0. The ply
    /// is analog to the turn counter in a game. It is incremented at the *beginning* of a turn.
    ///
    /// **State machine** - This function can be called at any stage.
    fn turn(&self) -> usize;

    /// Returns the next state after applying the given action and incrementing the turn counter.
    ///
    /// # Arguments
    ///  - `action` - The action to apply to this state
    ///
    /// **State machine** - This function can be called in the [StateStage::Await] stage. The state
    /// returned should be in the [StateStage::Await] or [StateStage::Terminal] stage.
    ///
    /// **Note: This function should not be reimplemented**
    fn apply_action(&self, action: &G::Action) -> Self;

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

/// A state that supplies probabilities for each action. This is used in games
/// where the outcome depends on chance, such as dice rolls.
pub trait ProbabilisticState<G: Game>: State<G> {
    type ProbabilityIterator: IntoIterator<Item = (G::State, f32)>;

    /// Return an iterator over all possible substates of this random state
    /// with their respective probability of occurring.
    fn substates(&self) -> Self::ProbabilityIterator;

    /// Return whether this state involves randomness.
    fn is_random_state(&self) -> bool;
}

/// Polarity is used to represent the evaluation direction according to the current player.
/// A positive polarity means that the current player is maximizing the evaluation, while a
/// negative polarity indicates that the current player is minimizing the evaluation.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Polarity {
    /// Positive polarity indicates the player is **maximizing** the evaluation.
    Positive = 1,
    /// Negative polarity indicates the player is **minimizing** the evaluation.
    Negative = -1,
}

impl Polarity {
    /// Returns the opposite of the current `Polarity`.
    ///
    /// If the current instance is `Positive`, it returns `Negative`, and vice versa.
    pub fn flip(&self) -> Self {
        match self {
            Polarity::Positive => Polarity::Negative,
            Polarity::Negative => Polarity::Positive,
        }
    }

    /// Returns the sign of the `Polarity` as an `i32`.
    ///
    /// This method returns `1` for `Positive` and `-1` for `Negative`.
    pub fn sign(&self) -> i32 {
        match self {
            Polarity::Positive => 1,
            Polarity::Negative => -1,
        }
    }
}

/// The `Team` trait encapsulates the concept of a team or a player in a two-player game.
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
/// Note: `SerializeAlias` and `DeserializeAlias` are used for serialization and deserialization support.
pub trait Team<G: Game<Team = Self>>:
    Copy + Clone + Debug + Eq + PartialEq + SerializeAlias + DeserializeAlias
{
    /// In the total order of teams, return the team after this one
    fn next(&self) -> Self;

    /// Returns the evaluation polarity of the team.
    /// A positive polarity means that the team is maximizing the evaluation, while a
    /// negative polarity indicates that the team is minimizing the evaluation.
    fn polarity(&self) -> Polarity;

    /// Returns the nth team that plays next
    /// If 0 is passed, then the current team is returned.
    #[inline]
    fn nth(&self, n: usize) -> Self {
        let mut team = *self;
        for _ in 0..n {
            team = team.next();
        }
        team
    }

    /// Returns the team that plays the current ply. For example, if the current ply is 1,
    /// then the starting team plays. If the current ply is 2, then the team after that plays.
    fn in_turn(turn: usize) -> Self {
        G::starting_team().nth(turn)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub enum TwoPlayerTeam {
    One,
    Two,
}

impl<G: Game<Team = TwoPlayerTeam>> Team<G> for TwoPlayerTeam {
    fn next(&self) -> Self {
        match self {
            TwoPlayerTeam::One => TwoPlayerTeam::Two,
            TwoPlayerTeam::Two => TwoPlayerTeam::One,
        }
    }

    fn polarity(&self) -> Polarity {
        match self {
            TwoPlayerTeam::One => Polarity::Positive,
            TwoPlayerTeam::Two => Polarity::Negative,
        }
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
/// Note: `SerializeAlias` and `DeserializeAlias` are used for serialization and deserialization support.
// Send and Sync required for `anyhow::Error`
pub trait Action<G: Game<Action = Self>>:
    Clone + Debug + PartialEq + Send + Sync + SerializeAlias + DeserializeAlias
{
}
