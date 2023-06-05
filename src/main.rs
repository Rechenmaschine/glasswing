#![allow(dead_code)]

use crate::agents::random_agent::RandomAgent;
use crate::agents::minimax_agent::MiniMaxAgent;
use crate::core::traits::*;
use crate::core::Contest;
use crate::games::counting_game::{CountingGame, CountingGameEvaluator, CountingTeam};

mod agents;
mod core;
mod games;

fn main() {
    let mut agent1: RandomAgent<CountingGame, _> = RandomAgent::default();
    let mut agent2: MiniMaxAgent<CountingGame, CountingGameEvaluator> = MiniMaxAgent::new(10, CountingGameEvaluator);

    let mut contest = Contest::new(CountingGame::initial_state(), &mut agent1, &mut agent2);
    while let Some((action, state)) = (&mut contest).next() {
        println!("{:?} increments by {}", CountingTeam::One.nth(state.ply() as isize - 1), action.increment);
        // print current state
        println!("Current count: {}", state.total());
    }
    println!("{:?}", contest.game_result());
}
