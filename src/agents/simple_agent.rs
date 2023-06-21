use crate::core::traits::*;
use anyhow::Error;
use std::marker::PhantomData;
use std::time::Duration;

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

impl<G: Game> Agent<G> for SimpleAgent<G> {
    fn recommend_action(&mut self, state: &G::State, _: Duration) -> Result<G::Action, Error> {
        Ok(state
            .actions()
            .get(0)
            .expect("No actions available")
            .clone())
    }
}
