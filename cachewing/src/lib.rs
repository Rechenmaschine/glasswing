#![cfg_attr(feature = "nightly", feature(new_uninit))]

mod quadratic_probing;
pub mod traits;

pub use traits::TranspositionHash;
pub use traits::TranspositionTable;

pub type QuadraticProbingTable64<K, V> =
    quadratic_probing::QuadraticProbingTableBase<K, V, traits::Entry64<K, V>>;
pub type QuadraticProbingTable<K, V, E> = quadratic_probing::QuadraticProbingTableBase<K, V, E>;
