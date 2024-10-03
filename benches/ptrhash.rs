use divan::Bencher;
use phf_benchmark::{ptrhash::PtrHashMap, SEED};
use rand::distributions::Standard;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;

fn main() {
    divan::main();
}

#[divan::bench]
fn build(bencher: Bencher<'_, '_>) {
    bencher
        .with_inputs(|| {
            let keys = Xoshiro256PlusPlus::seed_from_u64(SEED)
                .sample_iter::<usize, _>(Standard)
                .take(1000)
                .collect::<Vec<_>>();

            let values = Xoshiro256PlusPlus::seed_from_u64(SEED)
                .sample_iter::<usize, _>(Standard)
                .take(1000)
                .collect::<Vec<_>>();

            (keys, values)
        })
        .bench_values(|(keys, values)| PtrHashMap::new(keys, values));
}

#[divan::bench]
fn query(bencher: Bencher<'_, '_>) {
    bencher
        .with_inputs(|| {
            let keys = Xoshiro256PlusPlus::seed_from_u64(SEED)
                .sample_iter::<usize, _>(Standard)
                .take(1000)
                .collect::<Vec<_>>();

            let values = Xoshiro256PlusPlus::seed_from_u64(SEED)
                .sample_iter::<usize, _>(Standard)
                .take(1000)
                .collect::<Vec<_>>();

            // `ptr_hash::PtrHash::index_minimal(key)` panics
            // if `key` was not present during the construction of `PtrHash`.
            // So, we have to pick a key that was used in the PHF construction.
            let mut rng = rand::thread_rng();
            let key = *keys.choose(&mut rng).unwrap();

            (PtrHashMap::new(keys, values), key)
        })
        .bench_refs(|(map, key)| {
            map.get_entry(key);
        });
}
