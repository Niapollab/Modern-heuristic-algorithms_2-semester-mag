use rand::{rngs::StdRng, SeedableRng};

pub fn random_provider(random_seed: Option<u64>) -> StdRng {
    if let Some(seed) = random_seed {
        StdRng::seed_from_u64(seed)
    } else {
        StdRng::from_entropy()
    }
}
