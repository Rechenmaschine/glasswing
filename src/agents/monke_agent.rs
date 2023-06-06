use crate::core::traits::*;
use std::marker::PhantomData;

/// The simplest agent possible, which always recommends the first available action.
pub struct MonkeAgent<G: Game> {
    _marker: PhantomData<G>,
}

impl<G: Game> MonkeAgent<G> {
    pub fn new() -> Self {
        MonkeAgent {
            _marker: PhantomData,
        }
    }
}

impl<G: Game> Default for MonkeAgent<G> {
    fn default() -> Self {
        Self::new()
    }
}

impl<G: Game> Agent for MonkeAgent<G> {
    type Game = G;

    fn identifier() -> String {
        String::from("MonkeAgent")
    }

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
