use crate::core::traits::*;
use rand::prelude::ThreadRng;
use rand::seq::SliceRandom;
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

impl<G: Game, R: Rng> Agent for RandomAgent<G, R> {
    type Game = G;

    fn recommend_move(
        &mut self,
        state: &<<Self as Agent>::Game as Game>::State,
    ) -> <G as Game>::Action {
        state.actions().choose(&mut self.rng).unwrap().clone()
    }
}
