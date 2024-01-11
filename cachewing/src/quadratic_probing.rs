use std::mem;
use std::mem::MaybeUninit;
use crate::traits::{AlwaysReplacePolicy, Entry, Entry64, TranspositionHash, TranspositionTable};

const RETRIES: usize = 8;
const C1: usize = 1;
const C2: usize = 2;

pub struct QuadraticProbingTableBase<K, V, E> {
    entries: Box<[MaybeUninit<E>]>,
    capacity: usize,
    size: usize,
    mask: usize,
    _marker: std::marker::PhantomData<(K, V)>,
}

impl<K, V, E> QuadraticProbingTableBase<K, V, E>
    where
        K: TranspositionHash,
        E: Entry<Key=K, Value=V, RawKey=u64>,
{

    /// Creates a new table with the given number of slots.
    ///
    /// # Panics
    /// Panics if `slots` is not a power of two.
    pub fn new(slots: usize) -> Self {
        assert!(slots.is_power_of_two(), "capacity must be a power of two");
        #[cfg(feature = "nightly")]
        {
            let entries = Box::<[E]>::new_zeroed_slice(slots);
            QuadraticProbingTableBase { entries, capacity: slots, size: 0, mask: slots - 1, _marker: Default::default() }
        }
        #[cfg(not(feature = "nightly"))]
        {
            let mut vec = Vec::<MaybeUninit<E>>::with_capacity(slots);
            // SAFETY: Vec pointer is properly aligned. Vec is allocated with slots*mem::size_of::<MaybeUninit<E>>
            // bytes therefore there is enough space to set the length to slots. Also, the memory is
            // zeroed, therefore no random data is read.
            unsafe {
                std::ptr::write_bytes(vec.as_mut_ptr() as *mut u8, 0, slots * mem::size_of::<MaybeUninit<E>>());
                vec.set_len(slots);
            }
            let entries = vec.into_boxed_slice();
            QuadraticProbingTableBase { entries, capacity: slots, size: 0, mask: slots - 1, _marker: Default::default() }
        }
    }

    fn get(&self, k: &K) -> Option<&V> {
        let hash = k.hash();
        if hash == 0 { return None; } // 0 is reserved for empty entries

        let mut i = hash as usize & self.mask;
        let mut attempts = 0;

        while attempts < RETRIES {
            // SAFETY: All non-zero entries are initialized and we only read
            // entries that are non-zero. As for the length, i is always in
            // bounds since we mask it with the capacity.
            unsafe {
                let entry = self.entries.get_unchecked(i).assume_init_ref();
                if *entry.raw_key() == hash {
                    // Key found, therefore we return the value
                    return Some(&entry.value());
                }
                if *entry.raw_key() == 0 {
                    // Found an uninitialized entry, therefore key doesn't exist
                    return None;
                }
            };

            attempts += 1;
            i = (i + C1 * attempts + C2 * attempts * attempts) & self.mask;
        }
        // The entry was not found in the table after RETRIES accesses.
        // However, since we never try to write for more than RETRIES
        // times, we can safely assume that the entry is not in the table.
        None
    }

    fn insert(&mut self, k: K, v: V) -> Option<V> {
        let hash = k.hash();
        if hash == 0 { return None; } // 0 is reserved for empty entries

        let mut i = hash as usize & self.mask;
        let mut attempts = 0;

        while attempts < RETRIES {
            // SAFETY: All non-zero entries are initialized and we only use
            // entries that are non-zero. As for the length, i is always in
            // bounds since we mask it with the capacity.
            unsafe {
                let entry = self.entries.get_unchecked_mut(i).assume_init_mut();
                if *entry.raw_key() == hash {
                    // Key already exists, therefore we replace the value
                    // SAFETY: entry is initialized, therefore we can safely
                    // read from it and replace the value
                    return Some(entry.replace(v));
                }
                if *entry.raw_key() == 0 {
                    // We found an empty slot, therefore we insert the key-value pair
                    // SAFETY: we don't read from the uninitialized entry
                    *entry.raw_key_mut() = hash;
                    *entry.value_mut() = v;
                    self.size += 1;
                    return None;
                }
            };

            attempts += 1;
            i = (i + C1 * attempts + C2 * attempts * attempts) & self.mask;
        }
        // If we've reached here, it means we couldn't insert the
        // key-value pair after RETRIES attempts. This means that
        // the table is full in that region.
        None
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn size_in_memory(&self) -> usize {
        self.capacity * mem::size_of::<E>() + mem::size_of::<Self>()
    }

    pub fn load_factor(&self) -> f64 {
        self.size as f64 / self.capacity as f64
    }
}

impl<K, V, E> TranspositionTable<K, V> for QuadraticProbingTableBase<K, V, E>
    where K: TranspositionHash,
          E: Entry<Key=K, Value=V, RawKey=u64>
{
    fn get<'a>(&'a self, k: &K) -> Option<&'a V> where Entry64<K, V>: 'a {
        QuadraticProbingTableBase::get(self, k)
    }

    fn insert(&mut self, k: K, v: V) -> Option<V> {
        QuadraticProbingTableBase::insert(self, k, v)
    }
}

impl<K, V, E> AlwaysReplacePolicy for QuadraticProbingTableBase<K, V, E>{}