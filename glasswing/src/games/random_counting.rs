use crate::core::{Action, Evaluator, Game, State, TwoPlayerGameResult, TwoPlayerTeam};
use crate::prelude::ProbabilisticState;
use anyhow::Error;

// A game, in which two players both roll a die in their turn. Then they may
// move their piece forward by 1 or by x, where x is the number they rolled.
// the first person to reach exactly 100 wins. If a player overshoots 100, they
// automatically lose.
// we will use TwoPlayerTeam
#[derive(Debug, Clone)]
struct RandomCounting {
    state: RCState,
}

impl Game for RandomCounting {
    type State = RCState;
    type Action = MoveForwardAction;
    type Team = TwoPlayerTeam;
    type GameResult = TwoPlayerGameResult<Self>;

    const NAME: &'static str = "RandomCounting";

    fn new() -> Self {
        RandomCounting {
            state: Self::initial_state(),
        }
    }

    fn before_turn(&mut self) {
        self.state.roll = Some(rand::random::<u32>() % 6 + 1);
    }

    fn after_turn(&mut self) {
        self.state.roll = None;
    }

    fn current_state(&self) -> Self::State {
        self.state.clone()
    }

    fn apply_action(&mut self, action: &Self::Action) {
        self.state = self.state.apply_action(action);
    }

    fn initial_state() -> Self::State {
        RCState {
            positions: [0, 0],
            turn: 0,
            roll: None,
        }
    }

    fn starting_team() -> Self::Team {
        TwoPlayerTeam::One
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MoveForwardAction {
    steps: u32,
}

impl Action<RandomCounting> for MoveForwardAction {}

#[derive(Debug, Clone)]
struct RCState {
    positions: [u32; 2],
    turn: usize,
    roll: Option<u32>,
}

impl State<RandomCounting> for RCState {
    type ActionIterator = Vec<MoveForwardAction>;

    fn actions(&self) -> Self::ActionIterator {
        let mut actions = Vec::with_capacity(6);
        for i in 0..=self.roll.unwrap() {
            actions.push(MoveForwardAction { steps: i });
        }
        actions
    }

    fn turn(&self) -> usize {
        self.turn
    }

    fn apply_action(&self, action: &MoveForwardAction) -> Self {
        let mut new_state = self.clone();
        match self.team_to_move() {
            TwoPlayerTeam::One => {
                new_state.positions[0] += action.steps;
            }
            TwoPlayerTeam::Two => {
                new_state.positions[1] += action.steps;
            }
        }
        new_state.turn += 1;
        new_state
    }

    fn is_terminal(&self) -> bool {
        self.positions[0] >= 100 || self.positions[1] >= 100
    }

    fn game_result(&self) -> Option<TwoPlayerGameResult<RandomCounting>> {
        if self.positions[0] >= 100 {
            Some(TwoPlayerGameResult::Winner(TwoPlayerTeam::One))
        } else if self.positions[1] >= 100 {
            Some(TwoPlayerGameResult::Winner(TwoPlayerTeam::Two))
        } else {
            None
        }
    }
}

impl ProbabilisticState<RandomCounting> for RCState {
    type ProbabilityIterator = Vec<(RCState, f32)>;

    fn substates(&self) -> Self::ProbabilityIterator {
        // generate all substate with a possible roll
        debug_assert!(
            self.is_random_state(),
            "substates() called on non-random state"
        );

        let mut substates = Vec::with_capacity(6);
        for roll in 1..=6 {
            let mut new_state = self.clone();
            new_state.roll = Some(roll);
            substates.push((new_state, 1.0 / 6.0));
        }
        substates
    }

    fn is_random_state(&self) -> bool {
        self.roll.is_some()
    }
}

struct RCEval;

impl Evaluator<RandomCounting> for RCEval {
    fn evaluate(&self, state: &RCState) -> Result<f32, Error> {
        match state.game_result() {
            Some(TwoPlayerGameResult::Winner(TwoPlayerTeam::One)) => Ok(100.0),
            Some(TwoPlayerGameResult::Winner(TwoPlayerTeam::Two)) => Ok(-100.0),
            Some(TwoPlayerGameResult::Draw) => Ok(0.0),
            None => Ok(self.heuristic(state)),
        }
    }

    fn heuristic(&self, state: &RCState) -> f32 {
        let mut value = 0.0;
        value += state.positions[0] as f32;
        value -= state.positions[1] as f32;
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agents::expectiminimax::ExpectiMiniMaxAgent;
    use crate::agents::random_agent::RandomAgent;
    use crate::core::Match;

    #[test]
    fn test_alternating() {
        // init logger
        //builder().filter_level(log::LevelFilter::Debug).init();

        for _ in 0..100 {
            let mut match_: Match<RandomCounting> =
                Match::new(RandomAgent::default(), ExpectiMiniMaxAgent::new(7, RCEval));

            while let Some(a) = match_.next() {
                match a {
                    Ok((_before, _action, _after)) => {
                        //println!("{:?}", before.team_to_move());
                        //println!("{:?} -> {:?} -> {:?}", before, action, after);
                    }
                    Err(err) => {
                        println!("{:?}", err);
                        break;
                    }
                }
            }

            println!(
                "{:?} : score: {:?}",
                match_.game_result().unwrap(),
                match_.state().positions
            );
        }
    }
}
