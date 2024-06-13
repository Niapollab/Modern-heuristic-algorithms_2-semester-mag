use std::ops::{Add, Mul};

use rand::{distributions::Distribution, rngs::StdRng, Rng, SeedableRng};

pub trait RngDistributionExt {
    #[allow(dead_code)]
    fn distribute_by_key<
        T,
        I: Iterator<Item = T>,
        F: Fn(&T) -> G,
        G: Default + Copy + PartialOrd + Add<Output = G> + Mul<Output = G>,
        D: Distribution<G>,
    >(
        &mut self,
        elements: I,
        distribution: D,
        selector: F,
    ) -> Option<T>;
}

impl RngDistributionExt for StdRng {
    fn distribute_by_key<
        T,
        I: Iterator<Item = T>,
        F: Fn(&T) -> G,
        G: Default + Copy + PartialOrd + Add<Output = G> + Mul<Output = G>,
        D: Distribution<G>,
    >(
        &mut self,
        elements: I,
        distribution: D,
        selector: F,
    ) -> Option<T> {
        let mut probability_acc = Default::default();
        let pairs: Vec<(G, T)> = elements
            .map(|element| {
                let probability = selector(&element);
                probability_acc = probability_acc + probability;

                (probability_acc, element)
            })
            .collect();

        let size = pairs.len();
        if size == 0 {
            return None;
        }

        let random_value = self.sample(distribution) * probability_acc;
        for (index, (probability, element)) in pairs.into_iter().enumerate() {
            if random_value < probability || index == size - 1 {
                return Some(element);
            }
        }

        None
    }
}

pub fn random_provider(random_seed: Option<u64>) -> StdRng {
    if let Some(seed) = random_seed {
        StdRng::seed_from_u64(seed)
    } else {
        StdRng::from_entropy()
    }
}
