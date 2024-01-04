use crate::agents::Agent;
use crate::core::{Game, GwState};
use anyhow::Error;
use rand::prelude::{IteratorRandom, ThreadRng};
use rand::Rng;
use std::marker::PhantomData;

pub struct RandomAgent<G: Game, R: Rng> {
    rng: R,
    _game: PhantomData<G>,
}

impl<G: Game, R: Rng> RandomAgent<G, R> {
    pub fn new(rng: R) -> Self {
        RandomAgent {
            rng,
            _game: PhantomData,
        }
    }
}

impl<G: Game> Default for RandomAgent<G, ThreadRng> {
    fn default() -> Self {
        Self::new(rand::thread_rng())
    }
}

impl<G: Game, R: Rng> Agent<G> for RandomAgent<G, R> {
    fn select_action(&mut self, state: &G::State) -> Result<G::Action, Error> {
        Ok(state
            .actions()
            .into_iter()
            .choose(&mut self.rng)
            .unwrap()
            .clone())
    }
}
