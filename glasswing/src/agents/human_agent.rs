use crate::agents::Agent;
use crate::core::{Game, GwState};
use anyhow::Error;
use std::io;
use std::io::{BufReader, Stdin};
use std::marker::PhantomData;

pub struct HumanAgent<G, I> {
    input_stream: I,
    _marker: PhantomData<G>,
}

impl<G, I> HumanAgent<G, I>
where
    G: Game,
    I: io::BufRead,
{
    pub fn new(input_stream: I) -> Self {
        HumanAgent {
            input_stream,
            _marker: PhantomData,
        }
    }
}

impl<G, I> Agent<G> for HumanAgent<G, I>
where
    G: Game,
    G::State: std::fmt::Display,
    G::Action: std::fmt::Display,
    I: io::BufRead,
{
    fn select_action(&mut self, state: &G::State) -> Result<G::Action, Error> {
        println!("{}", state);
        println!("Select an action. The following moves are available: ");
        for (i, action) in state.actions().into_iter().enumerate() {
            println!("({}): {}", i, action);
        }
        loop {
            let mut input = String::new();
            self.input_stream.read_line(&mut input)?;
            if let Ok(idx) = input.trim().parse::<usize>() {
                if idx < state.count_actions() {
                    let action = state.actions().into_iter().nth(idx).unwrap();
                    return Ok(action);
                }
            }
            println!(
                "Enter a valid index between 0 and {}.",
                state.count_actions() - 1
            );
        }
    }
}

impl<G> Default for HumanAgent<G, BufReader<Stdin>>
where
    G: Game,
{
    fn default() -> Self {
        HumanAgent::new(BufReader::new(io::stdin()))
    }
}
