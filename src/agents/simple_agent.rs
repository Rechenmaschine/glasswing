use crate::core::traits::*;
use std::marker::PhantomData;

/// The simplest agent possible, which always recommends the first available action.
pub struct SimpleAgent<G: Game> {
    _marker: PhantomData<G>,
}

impl<G: Game> SimpleAgent<G> {
    pub fn new() -> Self {
        SimpleAgent {
            _marker: PhantomData,
        }
    }
}

impl<G: Game> Default for SimpleAgent<G> {
    fn default() -> Self {
        Self::new()
    }
}

impl<G: Game> Agent for SimpleAgent<G> {
    type Game = G;

    fn recommend_move(
        &mut self,
        state: &<<Self as Agent>::Game as Game>::State,
    ) -> <G as Game>::Action {
        state
            .actions()
            .get(0)
            .expect("No actions available")
            .clone()
    }
}
