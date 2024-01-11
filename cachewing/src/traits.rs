pub trait TranspositionHash<Target = u64> {
    fn hash(&self) -> Target;
}

/// A transposition table is a hash table that stores positions together with
/// associated data, such as evaluated scores, best moves, or node counts.
/// Transposition tables are commonly used to avoid re-computation of positions
/// that have already been seen before.
///
/// # Specification
/// A transposition table maps a key type `K` to a value of type `V`.
///
/// # Collision Errors
/// as specified in [chessprogramming.org/Transposition_Table]
///
/// **Type-1 Errors:**
/// Key collisions or type-1 errors are inherent in using signatures with far less
/// bits than required to encode all reachable positions. A key collision occurs
/// when two different positions map the same hash key or signature. When storing
/// only a partial key, the chance of a collision greatly increases.
///
/// **Type-2 Errors:**
/// Index collisions or type-2 errors, where different hash keys index same entries,
/// happen regularly. They require detection, realized by storing the signature as
/// part of the hash entry, to check whether a stored entry matches the position
/// while probing.
///
/// [chessprogramming.org/Transposition_Table]: https://www.chessprogramming.org/Transposition_Table
pub trait TranspositionTable<K, V, E = Entry64<K, V>>
    where E: Entry<Key=K, Value=V>
{
    /// Looks up a key in the table, and returns an associated value if it exists.
    /// Note, that there is no guarantee that the returned value is the most recently
    /// inserted value for the given key. See [AlwaysReplacePolicy] for a more strict
    /// constraint.
    ///
    /// If the key collides with another key in the table (type-1 error), then this
    /// function may yield a false positive.
    ///
    /// Under the premise of no hash collisions, this function always returns
    /// - `Some(&V)` if the associated key exists in the table
    /// - `None` if the key does not exist in the table
    fn get<'a>(&'a self, k: &K) -> Option<&'a V> where E: 'a;

    /// Attempts to insert a key-value pair into the table.
    ///
    /// If the key collides with another key in the table (type-1 error), then
    /// this function may falsely evict the existing key-value pair and replace
    /// it with the new key-value pair. It will then return the evicted Value.
    ///
    /// Under the premise of no hash collisions, this function attempts to insert
    /// the key and returns
    /// - `None` if insertion was successful and the key did not exist yet
    /// - `Some(old)` when the key already existed and was replaced by the new key
    /// - `None` if the key could not be inserted. This can happen if the table
    /// is full (type-2 error).
    fn insert(&mut self, k: K, v: V) -> Option<V>;
}

/// Marker trait for a replacement policy in a transposition table
/// where a previously inserted entry is always replaced by a new entry.
/// More specifically, if `insert(k, v1)` succeeds => a later call to
/// `insert(k, v2)` will replace `v1` with `v2` according to the policy specified
/// in the entry type `E`.
pub trait AlwaysReplacePolicy {}


/// An entry in a TranspositionTable
pub trait Entry {

    /// The key type of the entry, which will be used to look up the entry in the table.
    /// This type should hash to the `Self::RawKey` type.
    type Key: TranspositionHash<Self::RawKey>;

    /// The raw key type of the entry, as it is stored in the table.
    type RawKey;

    /// The value type of the entry, as it is stored in the table.
    type Value;

    /// Creates a new entry from the given key and value.
    fn new(k: Self::Key, v: Self::Value) -> Self;

    /// Returns a reference to the raw key of the entry.
    fn raw_key(&self) -> &Self::RawKey;

    /// Returns a mutable reference to the raw key of the entry.
    fn raw_key_mut(&mut self) -> &mut Self::RawKey;

    /// Returns a reference to the value of the entry.
    fn value(&self) -> &Self::Value;

    /// Returns a mutable reference to the value of the entry.
    fn value_mut(&mut self) -> &mut Self::Value;

    /// Replaces the value of the entry with the given value and returns the old value.
    fn replace(&mut self, v: Self::Value) -> Self::Value;

    /// Destroys the entry and returns the raw key and value.
    /// This is useful for moving the entry out of the table.
    fn take(self) -> (Self::RawKey, Self::Value);
}


pub struct Entry64<K, V> {
    key: u64,
    value: V,
    _marker: std::marker::PhantomData<K>,
}

impl<K: TranspositionHash, V> Entry for Entry64<K, V> {
    type Key = K;
    type RawKey = u64;
    type Value = V;

    #[inline(always)]
    fn new(k: Self::Key, v: Self::Value) -> Self {
        Entry64 {
            key: k.hash(),
            value: v,
            _marker: std::marker::PhantomData,
        }
    }

    #[inline(always)]
    fn raw_key(&self) -> &Self::RawKey {
        &self.key
    }

    #[inline(always)]
    fn raw_key_mut(&mut self) -> &mut Self::RawKey {
        &mut self.key
    }

    #[inline(always)]
    fn value(&self) -> &Self::Value {
        &self.value
    }

    #[inline(always)]
    fn value_mut(&mut self) -> &mut Self::Value {
        &mut self.value
    }

    #[inline(always)]
    fn replace(&mut self, v: Self::Value) -> Self::Value {
        std::mem::replace(&mut self.value, v)
    }

    #[inline(always)]
    fn take(self) -> (Self::RawKey, Self::Value) {
        (self.key, self.value)
    }
}


pub trait EntryBasedTranspositionTable<E: Entry> {
    fn get_entry(&self, k: &E::Key) -> Option<&E>;
    fn insert_entry(&mut self, k: E::Key, v: E::Value) -> Option<E>;
}


impl<Table, E, K, V> TranspositionTable<K, V, E> for Table
    where Table: EntryBasedTranspositionTable<E>,
          E: Entry<Key=K, Value=V>,
          K: TranspositionHash<E::RawKey>,
{
    #[inline]
    fn get<'a>(&'a self, k: &K) -> Option<&'a V> where E: 'a {
        Table::get_entry(self, k).map(|e| e.value())
    }

    #[inline]
    fn insert(&mut self, k: K, v: V) -> Option<V> {
        Table::insert_entry(self, k, v).map(|e| e.take()).map(|(_, v)| v)
    }
}