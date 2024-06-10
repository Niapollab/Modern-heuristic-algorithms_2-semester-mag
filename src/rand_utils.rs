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
        let mut elements: Vec<T> = elements.collect();

        let size = elements.len();
        if size < 1 {
            return None;
        }

        elements.sort_by(|first, second| selector(first).partial_cmp(&selector(second)).unwrap());

        let mut distribution_sum = selector(&elements[0]);
        for element in elements.iter().skip(1) {
            distribution_sum = distribution_sum + selector(element);
        }

        let random_value = self.sample(distribution);
        let mut acc_probability: G = Default::default();

        for (index, element) in elements.into_iter().enumerate() {
            let probability = selector(&element) / distribution_sum;
            acc_probability = acc_probability + probability;

            if index == size - 1 || random_value < acc_probability {
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
