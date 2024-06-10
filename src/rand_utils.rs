use std::ops::{Add, Div};

use rand::{distributions::Distribution, rngs::StdRng, Rng, SeedableRng};

pub trait RngDistributionExt {
    #[allow(dead_code)]
    fn distribute_by_key<
        T,
        I: Iterator<Item = T>,
        F: Fn(&T) -> G,
        G: Default + Copy + PartialOrd + Add<Output = G> + Div<Output = G>,
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
        G: Default + Copy + PartialOrd + Add<Output = G> + Div<Output = G>,
        D: Distribution<G>,
    >(
        &mut self,
        elements: I,
        distribution: D,
        selector: F,
    ) -> Option<T> {
        let mut distribution_sum = Default::default();
        let mut pairs: Vec<(G, T)> = elements
            .map(|element| {
                let probability = selector(&element);
                distribution_sum = distribution_sum + probability;

                (probability, element)
            })
            .collect();

        let size = pairs.len();
        if size < 1 {
            return None;
        }

        pairs.sort_by(|(first, _), (second, _)| first.partial_cmp(second).unwrap());

        let random_value = self.sample(distribution);
        let mut acc_probability: G = Default::default();

        for (index, (probability, element)) in pairs.into_iter().enumerate() {
            let probability = probability / distribution_sum;
            acc_probability = acc_probability + probability;

            if random_value < acc_probability || index == size - 1 {
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
