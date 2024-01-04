use crate::agents::Agent;
use crate::core::Game;
use anyhow::Error;
use std::marker::PhantomData;

pub struct FunctionalAgent<G, F> {
    f: F,
    _marker: PhantomData<G>,
}

impl<G, F> FunctionalAgent<G, F>
where
    G: Game,
    F: FnMut(&G::State) -> Result<G::Action, Error>,
{
    pub fn new(f: F) -> Self {
        FunctionalAgent {
            f,
            _marker: PhantomData,
        }
    }
}

impl<G, F> Agent<G> for FunctionalAgent<G, F>
where
    G: Game,
    F: FnMut(&G::State) -> Result<G::Action, Error>,
{
    fn select_action(&mut self, state: &G::State) -> Result<G::Action, Error> {
        (self.f)(state)
    }
}

impl<G, F> Agent<G> for F
where
    G: Game,
    F: FnMut(&G::State) -> Result<G::Action, Error>,
{
    fn select_action(&mut self, state: &G::State) -> Result<G::Action, Error> {
        (self)(state)
    }
}
