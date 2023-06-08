#![allow(dead_code)]

use crate::agents::minimax_agent::MiniMaxAgent;
use crate::agents::random_agent::RandomAgent;
use crate::core::{ContestBuilder, Game};
use crate::core::player::PlayerBuilder;
use crate::games::counting_game::{CountingGame, CountingGameEvaluator};

mod agents;
mod core;
mod games;

fn main() {
    let agent1: RandomAgent<CountingGame, _> = RandomAgent::default();
    let agent2: MiniMaxAgent<CountingGame, CountingGameEvaluator> =
        MiniMaxAgent::new(10, CountingGameEvaluator);

    let mut contest = ContestBuilder::new()
        .initial_state(CountingGame::initial_state())
        .player_starts(
            PlayerBuilder::new()
                .name("Random")
                .agent(agent1)
                .build()
                .unwrap())
        .plays_aginst(
            PlayerBuilder::new()
                .name("MiniMax")
                .agent(agent2)
                .build()
                .unwrap())
        .build()
        .unwrap();

    contest.play();
    contest.history()
        .save_to("history.json")
        .expect("Failed to save history to file");
}
