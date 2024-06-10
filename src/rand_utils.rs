use std::ops::{Add, Div};

use rand::{distributions::Distribution, rngs::StdRng, Rng, SeedableRng};

pub trait RngDistributionExt {
    #[allow(dead_code)]
    fn distribute<T: Copy + PartialOrd<T> + Add<Output = T> + Div<Output = T>, D: Distribution<T>>(
        &mut self,
        elements: &[T],
        distribution: D,
    ) -> Option<usize>;
}

impl RngDistributionExt for StdRng {
    fn distribute<
        T: Copy + PartialOrd<T> + Add<Output = T> + Div<Output = T>,
        D: Distribution<T>,
    >(
        &mut self,
        elements: &[T],
        distribution: D,
    ) -> Option<usize> {
        let size = elements.len();
        if size < 1 {
            return None;
        }
        let mut elements: Vec<T> = elements.iter().copied().collect();

        let mut distribution_sum = elements[0];
        for element in elements.iter().skip(1) {
            distribution_sum = distribution_sum + *element;
        }

        elements[0] = elements[0] / distribution_sum;
        for index in 1..size {
            elements[index] = elements[index] / distribution_sum;
            elements[index] = elements[index] + elements[index - 1];
        }
        elements.sort_by(|first, second| first.partial_cmp(second).unwrap());

        let random_value = self.sample(distribution);
        for (index, element) in elements.iter().enumerate() {
            if random_value < *element {
                return Some(index);
            }
        }

        Some(size - 1)
    }
}

pub fn random_provider(random_seed: Option<u64>) -> StdRng {
    if let Some(seed) = random_seed {
        StdRng::seed_from_u64(seed)
    } else {
        StdRng::from_entropy()
    }
}
