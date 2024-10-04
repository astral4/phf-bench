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
    values: Vec<V>,
}

impl<K: KeyT, V> PtrHashMap<K, V> {
    /// # Panics
    /// Panics if `keys.len() != values.len()`.
    #[must_use]
    pub fn new(keys: &[K], values: Vec<V>) -> Self {
        assert_eq!(keys.len(), values.len());

        let hasher: PtrHash<_, EliasFano, InnerHasher> =
            PtrHash::new(keys, PtrHashParams::default());

        let mut entries: Vec<_> = zip(keys, values).collect();
        entries.sort_by_cached_key(|(k, _)| hasher.index_minimal(k));
        let values = entries.into_iter().map(|(_, v)| v).collect();

        Self { hasher, values }
    }

    /// # Panics
    /// This function may panic if `key` was not present during the construction of this instance of `PtrHashMap`.
    pub fn get_entry(&self, key: &K) -> &V
    where
        K: PartialEq,
    {
        let index = self.hasher.index_minimal(key);
        &self.values[index]
    }
}
