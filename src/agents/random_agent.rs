use crate::core::traits::*;
use crate::core::Error;
use rand::prelude::ThreadRng;
use rand::seq::SliceRandom;
use rand::Rng;
use std::marker::PhantomData;
use std::time::Duration;

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

impl<G: Game, R: Rng> Agent for RandomAgent<G, R> {
    type Game = G;

    fn recommend_action(
        &mut self,
        state: &<G as Game>::State,
        _: Duration,
    ) -> Result<<G as Game>::Action, Error> {
        Ok(state.actions().choose(&mut self.rng).unwrap().clone())
    }
}
