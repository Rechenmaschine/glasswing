use std::fmt::Debug;

pub trait Agent {
    type Game: Game;

    /*
    /// Returns the team that the agent is playing for
    fn team(&self) -> <<Self as Agent>::Game as Game>::Team;
    */

    /// Returns the recommended move for the given state
    fn recommend_move(
        &mut self,
        state: &<<Self as Agent>::Game as Game>::State,
    ) -> <<Self as Agent>::Game as Game>::Action;
}

pub trait Evaluator<G: Game> {
    /// Evaluates the given state for a two-player game.
    ///
    /// The magnitude of the score indicates how likely a player is to win.
    /// - A positive score indicates that the first player is winning
    /// - A negative score indicates that the second player is winning
    /// - A score of 0 indicates that the game is likely a draw
    fn evaluate(&self, state: &G::State) -> f32;
}

/// Trait for a game, that links together all necessary types for a game
pub trait Game: Sized {
    type State: State<Self>;
    type Action: Action<Self>;
    type Team: Team<Self>;

    type GameResult: GameResult<Self>;

    /// Returns the initial state of the game
    fn initial_state() -> Self::State;
}

pub trait GameResult<G: Game>: Clone + Debug {
    /// The winner of the game
    fn winner(&self) -> Option<G::Team>;

    /// The loser of the game
    fn loser(&self) -> Option<G::Team> {
        self.winner().map(|t| t.next())
    }

    /// Returns true, if the game is a draw
    fn is_draw(&self) -> bool;
}

pub trait State<G: Game<State=Self>>: Clone + Debug {
    /// Returns a vector of all possible actions that can be taken from this state
    fn actions(&self) -> Vec<G::Action>;

    /// Returns a vector of all possible substates that can be reached from this state
    fn substates(&self) -> Vec<Self> {
        self.actions().iter().map(|a| self.next_state(a)).collect()
    }

    /// Returns the team whose turn it is to play
    fn current_team(&self) -> G::Team;

    /// Returns the current ply of the game. A ply is a half move, ie. the action of one player.
    /// The initial state has ply 0.
    fn ply(&self) -> usize;

    /// Returns the next state after applying the given action and advancing the ply
    ///
    /// **Note: You probably should not reimplement this function**
    fn next_state(&self, action: &G::Action) -> Self {
        self.apply_action(action).advance_ply()
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

pub trait Team<G: Game<Team=Self>>: Copy + Clone + Debug {
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
}

pub trait Action<G: Game<Action=Self>>: Clone + Debug {}
