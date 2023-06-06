#![allow(dead_code)]

use crate::agents::minimax_agent::MiniMaxAgent;
use crate::agents::random_agent::RandomAgent;
use crate::core::traits::*;
use crate::core::Contest;
use crate::games::counting_game::{CountingGame, CountingGameEvaluator};

mod agents;
mod core;
mod games;

fn main() {
    let mut agent1: RandomAgent<CountingGame, _> = RandomAgent::default();
    let mut agent2: MiniMaxAgent<CountingGame, CountingGameEvaluator> =
        MiniMaxAgent::new(10, CountingGameEvaluator);
    //let mut agent2: RandomAgent<CountingGame, _> = RandomAgent::default();

    let mut contest = Contest::new(CountingGame::initial_state(), &mut agent1, &mut agent2);
    while let Some((old, action, curr)) = (&mut contest).next() {
        println!(
            "{:?} -> player {:?} increments by {} -> {:?}",
            old,
            curr.current_team(),
            action.increment,
            curr
        );
    }
    println!("{:?}", contest.game_result());
}
