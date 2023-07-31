use std::collections::HashMap;
use std::hash::{BuildHasher, Hasher};
use std::marker::PhantomData;

/// A transposition table that stores that stores associative data `D` for
/// a given state `S`.
pub trait TranspositionTable<S, D>
where
    S: DirectHash,
{
    /// Return true if the state is in the table, otherwise false.
    fn contains(&self, state: &S) -> bool {
        self.get(state).is_some()
    }

    /// Return Some(data) if the state is in the table, otherwise None.
    fn get(&self, state: &S) -> Option<&D>;

    /// Insert the state and it's associated data into the table, returning
    /// replacing and returning previous data if it existed.
    fn insert(&mut self, state: &S, data: D) -> Option<D>;

    /// Insert the state and it's associated data into the table **without**
    /// checking if state is already in the table.
    unsafe fn insert_unchecked(&mut self, state: &S, data: D) -> Option<D> {
        self.insert(state, data)
    }

    /// Return the number of elements the table can hold without reallocating.
    //TODO change such that capacity is hard limit
    fn capacity(&self) -> usize;

    /// Return the number of elements currently in the table.
    fn size(&self) -> usize;
}

/// A trait for types that can be hashed directly to a `u64`. This is useful
/// for types that already include a hash value, such as a Zobrist hash.
///
/// A good implementation should be fast to compute, have a low collision
/// rate, and be well-distributed.
pub trait DirectHash {
    fn get_hash(&self) -> u64;
}

/// A transposition table that stores that stores associative data `D` for
/// a given state `S`. This implementation uses a HashMap that stores
/// the hash value of the state as the key and the data as the value to
/// save on memory. However, this means that hash collisions are possible.
///
/// By default, this implementation uses the SplitMix64 algorithm to
/// redistribute the keys, thus avoiding clustering and improving performance.
pub struct TranspositionHashMap<S, D, H: BuildHasher = SplitMix64>
where
    S: DirectHash,
{
    table: HashMap<u64, D, H>,
    _phantom: PhantomData<S>,
    //TODO max size
}

impl<S, D, H> TranspositionTable<S, D> for TranspositionHashMap<S, D, H>
where
    S: DirectHash,
    H: BuildHasher,
{
    fn contains(&self, state: &S) -> bool {
        let hash = state.get_hash();
        self.table.contains_key(&hash)
    }

    fn get(&self, state: &S) -> Option<&D> {
        let hash = state.get_hash();
        self.table.get(&hash)
    }

    fn insert(&mut self, state: &S, data: D) -> Option<D> {
        let hash = state.get_hash();
        self.table.insert(hash, data)
    }

    fn capacity(&self) -> usize {
        self.table.capacity()
    }

    fn size(&self) -> usize {
        self.table.len()
    }
}

impl<S, D> TranspositionHashMap<S, D, SplitMix64>
where
    S: DirectHash,
{
    /// Create a new transposition table with no capacity limit and SplitMixHasher.
    pub fn new() -> Self {
        Self {
            table: HashMap::with_hasher(SplitMix64::default()),
            _phantom: Default::default(),
        }
    }

    /// Create a new transposition table with the given capacity and SplitMixHasher.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            table: HashMap::with_capacity_and_hasher(capacity, SplitMix64::default()),
            _phantom: Default::default(),
        }
    }
}

impl<S, D, H> TranspositionHashMap<S, D, H>
where
    S: DirectHash,
    H: BuildHasher + Default,
{
    /// Create a new transposition table without a capacity limit using the given hasher.
    pub fn with_hasher(hasher: H) -> Self {
        Self {
            table: HashMap::with_hasher(hasher),
            _phantom: Default::default(),
        }
    }

    /// Create a new transposition table with the given hasher and capacity.
    pub fn with_capacity_and_hasher(capacity: usize, hasher: H) -> Self {
        Self {
            table: HashMap::with_capacity_and_hasher(capacity, hasher),
            _phantom: Default::default(),
        }
    }
}

impl<S, D, H> std::fmt::Display for TranspositionHashMap<S, D, H>
where
    S: DirectHash,
    H: BuildHasher + Default,
    D: std::fmt::Display,
{
    // write the elements
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{")?;
        for (key, value) in self.table.iter() {
            write!(f, "{}={}, ", key, value)?;
        }
        write!(f, "}}")
    }
}

impl<S, D, H> std::fmt::Debug for TranspositionHashMap<S, D, H>
where
    S: DirectHash,
    H: BuildHasher + Default,
    D: std::fmt::Debug,
{
    // write the elements
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "TranspositionHashMap {{capacity: {}, ", self.capacity())?;
        for (key, value) in self.table.iter() {
            write!(f, "{}={:?}, ", key, value)?;
        }
        write!(f, "}}")
    }
}

/// A hasher that passes through the value without hashing anything.
/// This is useful for types that already include a hash value, such as
/// a Zobrist hash and don't need to be hashed again.
pub struct TransparentHasher {
    value: u64,
}

impl TransparentHasher {
    pub fn new() -> Self {
        TransparentHasher { value: 0 }
    }
}

impl Hasher for TransparentHasher {
    fn finish(&self) -> u64 {
        self.value
    }

    fn write(&mut self, _: &[u8]) {
        unimplemented!("This function may not be called.")
    }

    fn write_u64(&mut self, i: u64) {
        self.value = i;
    }
}

impl BuildHasher for TransparentHasher {
    type Hasher = TransparentHasher;

    fn build_hasher(&self) -> Self::Hasher {
        TransparentHasher { value: 0 }
    }
}

#[inline]
const fn splitmix(mut x: u64) -> u64 {
    //x ^= x >> 30;
    x ^= x.wrapping_shr(30);
    //x *= 0xbf58476d1ce4e5b9u64;
    x = x.wrapping_mul(0xbf58476d1ce4e5b9u64);
    //x ^= x >> 27;
    x ^= x.wrapping_shr(27);
    //x *= 0x94d049bb133111ebu64;
    x = x.wrapping_mul(0x94d049bb133111ebu64);
    //x ^= x >> 31;
    x ^= x.wrapping_shr(31);
    return x;
}

/// A hasher that uses the SplitMix64 algorithm to redistribute the keys,
/// specifically to avoid clustering in `TranspositionHashMap`.
pub struct SplitMix64 {
    value: u64,
}

impl Hasher for SplitMix64 {
    fn finish(&self) -> u64 {
        self.value
    }

    fn write(&mut self, _: &[u8]) {
        unimplemented!("This function may not be called.")
    }

    fn write_u64(&mut self, i: u64) {
        self.value = splitmix(i);
    }
}

impl BuildHasher for SplitMix64 {
    type Hasher = SplitMix64;

    fn build_hasher(&self) -> Self::Hasher {
        SplitMix64 { value: 0 }
    }
}

impl Default for SplitMix64 {
    fn default() -> Self {
        SplitMix64 { value: 0 }
    }
}
