use ahash::RandomState;
use core::hash::Hash;
use core::iter::zip;
use ptr_hash::hash::Hasher;
use ptr_hash::pack::EliasFano;
use ptr_hash::{KeyT, PtrHash, PtrHashParams};

#[derive(Clone)]
struct InnerHasher;

impl<T: Hash> Hasher<T> for InnerHasher {
    type H = u64;
    fn hash(x: &T, seed: u64) -> Self::H {
        RandomState::with_seed(usize::try_from(seed).unwrap()).hash_one(x)
    }
}

// `PtrHash` doesn't impl Debug
#[allow(missing_debug_implementations)]
pub struct PtrHashMap<K: KeyT, V> {
    hasher: PtrHash<K, EliasFano, InnerHasher>,
    entries: Vec<(K, V)>,
}

impl<K: KeyT, V> PtrHashMap<K, V> {
    /// # Panics
    /// Panics if `keys.len() != values.len()`.
    #[must_use]
    pub fn new(keys: Vec<K>, values: Vec<V>) -> Self {
        assert_eq!(keys.len(), values.len());

        let hasher: PtrHash<_, EliasFano, InnerHasher> =
            PtrHash::new(&keys, PtrHashParams::default());

        let mut entries: Vec<_> = zip(keys, values).collect();

        entries.sort_by_cached_key(|(k, _)| hasher.index_minimal(k));

        Self { hasher, entries }
    }

    pub fn get_entry(&self, key: &K) -> Option<(&K, &V)>
    where
        K: PartialEq,
    {
        let index = self.hasher.index_minimal(key);
        let entry = &self.entries[index];

        if entry.0 == *key {
            Some((&entry.0, &entry.1))
        } else {
            None
        }
    }
}
