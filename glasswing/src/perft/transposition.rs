use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use crate::core::Game;

/// A simple transposition table that stores the perft value for a given state.
/// This implementation does not store the state itself for speed and space
/// efficiency, so keep in mind that hash collisions are possible.
pub struct TranspositionTable<G: Game>
    where G::State: Hash + Eq
{
    table: HashMap<u64, usize>,
    phantom: std::marker::PhantomData<G>,
}

impl<G: Game> TranspositionTable<G>
    where G::State: Hash + Eq
{
    pub fn new() -> Self {
        TranspositionTable {
            table: HashMap::new(),
            phantom: std::marker::PhantomData,
        }
    }

    pub fn get(&self, state: &G::State) -> Option<usize> {
        let mut hasher = DirectHasher::new();
        state.hash(&mut hasher);
        let hash = hasher.finish();
        self.table.get(&hash).copied()
    }

    pub fn insert(&mut self, state: G::State, perft: usize) {
        let mut hasher = DirectHasher::new();
        state.hash(&mut hasher);
        let hash = hasher.finish();
        self.table.insert(hash, perft);
    }
}

// passes through the value without hashing anything
struct DirectHasher {
    value: u64,
}

impl DirectHasher {
    fn new() -> Self {
        DirectHasher {
            value: 0,
        }
    }
}

impl Hasher for DirectHasher {
    fn finish(&self) -> u64 {
        self.value
    }

    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.write_u8(*byte);
        }
    }

    fn write_u8(&mut self, i: u8) {
        self.value <<= 8;
        self.value |= i as u64;
    }

    fn write_u64(&mut self, i: u64) {
        self.value = i;
    }
}