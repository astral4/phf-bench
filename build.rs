use core::iter::zip;
use phf_codegen::Map;
use rand::distributions::Standard;
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;
use std::env::var;
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

const SEED: u64 = 42;
const ENTRIES: usize = 1000;

fn main() -> Result<(), Box<dyn Error>> {
    let mut map = Map::new();

    let keys = Xoshiro256PlusPlus::seed_from_u64(SEED)
        .sample_iter::<usize, _>(Standard)
        .take(ENTRIES);

    let values = Xoshiro256PlusPlus::seed_from_u64(SEED)
        .sample_iter::<usize, _>(Standard)
        .take(ENTRIES);

    for (key, value) in zip(keys, values) {
        map.entry(key, &value.to_string());
    }

    let path = Path::new(&var("OUT_DIR")?).join("phf_map_codegen.rs");
    let mut file = BufWriter::new(File::create(path)?);

    write!(
        &mut file,
        "#[allow(clippy::unreadable_literal)]\npub const PHF_MAP: phf::Map<usize, usize> = {};",
        map.build()
    )?;

    Ok(())
}
