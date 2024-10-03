use core::iter::zip;
use divan::Bencher;
use phf_benchmark::{PHF_MAP, SEED};
use phf_codegen::Map;
use rand::distributions::Standard;
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;

fn main() {
    divan::main();
}

#[divan::bench]
fn build(bencher: Bencher<'_, '_>) {
    let mut map = Map::new();

    let keys = Xoshiro256PlusPlus::seed_from_u64(SEED)
        .sample_iter::<usize, _>(Standard)
        .take(1000);

    let values = Xoshiro256PlusPlus::seed_from_u64(SEED)
        .sample_iter::<usize, _>(Standard)
        .take(1000);

    for (key, value) in zip(keys, values) {
        map.entry(key, &value.to_string());
    }

    bencher.with_inputs(|| &map).bench_values(|map| map.build());
}

#[divan::bench]
fn query(bencher: Bencher<'_, '_>) {
    bencher
        .with_inputs(rand::random)
        .bench_refs(|key| PHF_MAP.get(key));
}
