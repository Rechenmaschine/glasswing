use crate::agents::Agent;
use crate::core::state::*;
use crate::core::{Game, MatchError};
use anyhow::Error;
use std::marker::PhantomData;

/// The simplest agent possible, which always recommends the first available action.
#[derive(Default)]
pub struct SimpleAgent<G> {
    _marker: PhantomData<G>,
}

impl<G: Game> SimpleAgent<G> {
    #[inline]
    pub fn new() -> Self {
        SimpleAgent {
            _marker: PhantomData,
        }
    }
}

impl<G: Game> Agent<G> for SimpleAgent<G> {
    #[inline]
    fn select_action(&mut self, state: &G::State) -> Result<G::Action, Error> {
        state
            .actions()
            .into_iter()
            .next()
            .ok_or_else(|| MatchError::<G>::NoAvailableActions(state.clone()).into())
    }
}
