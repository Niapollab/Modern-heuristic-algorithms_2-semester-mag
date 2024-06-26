use rand::{distributions::Standard, rngs::StdRng, Rng};

use crate::{
    models::{AdjMatrix, Solver, VisitedVecExt, Way, WayVecExt},
    rand_utils::{random_provider, RngDistributionExt},
};

struct AlgorithmState<'a> {
    adj_matrix: &'a AdjMatrix<u32>,
    reverse_distance_matrix: Vec<Vec<f64>>,

    max_iteration: u32,
    population_size: usize,
    random_provider: StdRng,

    pheromone_importance: f64,
    destination_importance: f64,
    pheromone_intensity: f64,
    pheromone_evaporation: f64,

    iteration: u32,
    pheromone_matrix: Vec<Vec<f64>>,
    probability_matrix: Vec<Vec<f64>>,
    best_way: Option<Way<'a>>,
}

impl<'a> Iterator for AlgorithmState<'a> {
    type Item = Way<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iteration >= self.max_iteration {
            return None;
        }
        self.iteration += 1;

        self.recalculate_probability_matrix();
        let population = self.build_population();

        self.spread_pheromone(&population);

        let iteration_best_way = population.into_iter().min().unwrap();
        let best_way = self.global_best_way(iteration_best_way);

        Some(best_way)
    }
}

impl<'a> AlgorithmState<'a> {
    #[inline]
    fn recalculate_probability_matrix(&mut self) {
        let nodes_count = self.adj_matrix.len();

        for row in 0..nodes_count {
            for column in 0..nodes_count {
                let first = self.pheromone_matrix[row][column].powf(self.pheromone_importance);
                let second =
                    self.reverse_distance_matrix[row][column].powf(self.destination_importance);

                self.probability_matrix[row][column] = first * second;
            }
        }
    }

    #[inline]
    fn build_population(&mut self) -> Vec<Way<'a>> {
        let population_size = self.population_size;
        let mut ant_ways: Vec<Way<'a>> = Vec::with_capacity(self.population_size);

        for _ in 0..population_size {
            let way = self.generate_ant_way();
            ant_ways.push(way);
        }

        ant_ways
    }

    #[inline]
    fn generate_ant_way(&mut self) -> Way<'a> {
        let adj_matrix = self.adj_matrix;
        let nodes_count = adj_matrix.len();

        let start_node = self.random_provider.gen_range(0..nodes_count);
        let mut node = start_node;

        let mut visited = vec![false; nodes_count];
        let mut way = Vec::with_capacity(nodes_count);

        loop {
            visited[node] = true;
            way.push(node);

            node = match self.find_next_node(node, &mut visited) {
                Some(next_node) => next_node,
                None => break,
            };
        }
        way.push(start_node);

        Way::new(adj_matrix, way)
    }

    #[inline]
    fn find_next_node(&mut self, node: usize, visited: &mut Vec<bool>) -> Option<usize> {
        self.random_provider
            .distribute_by_key(visited.available_neighbors(), Standard, |key| {
                self.probability_matrix[node][*key]
            })
    }

    #[inline]
    fn spread_pheromone(&mut self, population: &Vec<Way<'a>>) {
        const MIN_PHEROMONE_VALUE: f64 = 1e-5;

        for row in &mut self.pheromone_matrix {
            for element in row {
                *element *= self.pheromone_evaporation;
            }
        }

        for ant in population {
            let stripped_way_score = ant.score() as f64;
            for (from, to) in ant.way().iter_edges() {
                self.pheromone_matrix[from][to] += self.pheromone_intensity / stripped_way_score;
            }
        }

        for row in &mut self.pheromone_matrix {
            for element in row {
                if *element < MIN_PHEROMONE_VALUE {
                    *element = MIN_PHEROMONE_VALUE;
                }
            }
        }
    }

    #[inline]
    fn global_best_way(&mut self, candidate: Way<'a>) -> Way<'a> {
        match self.best_way.as_mut() {
            Some(way) if candidate <= *way => {
                *way = candidate.clone();
                candidate
            }
            None => {
                self.best_way = Some(candidate.clone());
                candidate
            }
            Some(way) => way.clone(),
        }
    }
}

pub struct AntQSolver {
    max_iteration: u32,
    population_size: usize,
    random_seed: Option<u64>,
    pheromone_importance: f64,
    destination_importance: f64,
    pheromone_intensity: f64,
    pheromone_evaporation: f64,
}

impl AntQSolver {
    pub fn new(
        max_iteration: u32,
        population_size: usize,
        random_seed: Option<u64>,
        pheromone_importance: f64,
        destination_importance: f64,
        pheromone_intensity: f64,
        pheromone_evaporation: f64,
    ) -> Self {
        Self {
            max_iteration,
            population_size,
            random_seed,
            pheromone_importance,
            destination_importance,
            pheromone_intensity,
            pheromone_evaporation,
        }
    }
}

impl Solver for AntQSolver {
    fn solve<'a>(&self, adj_matrix: &'a AdjMatrix<u32>) -> Way<'a> {
        const PHEROMONE_INIT_STATE: f64 = 1.0;

        let nodes_count = adj_matrix.len();
        let reverse_distance_matrix = AntQSolver::build_reverse_distance_matrix(adj_matrix);

        let state = AlgorithmState {
            adj_matrix,
            reverse_distance_matrix,
            random_provider: random_provider(self.random_seed),
            max_iteration: self.max_iteration,
            population_size: self.population_size,
            pheromone_importance: self.pheromone_importance,
            destination_importance: self.destination_importance,
            pheromone_intensity: self.pheromone_intensity,
            pheromone_evaporation: self.pheromone_evaporation,
            iteration: 0,
            pheromone_matrix: vec![vec![PHEROMONE_INIT_STATE; nodes_count]; nodes_count],
            probability_matrix: vec![vec![0.0; nodes_count]; nodes_count],
            best_way: None,
        };

        state.last().unwrap()
    }
}

impl AntQSolver {
    #[inline]
    fn build_reverse_distance_matrix(adj_matrix: &AdjMatrix<u32>) -> Vec<Vec<f64>> {
        let nodes_count = adj_matrix.len();
        let mut reverse_distance_matrix = vec![vec![0.0; nodes_count]; nodes_count];

        for row in 0..nodes_count {
            for column in 0..nodes_count {
                reverse_distance_matrix[row][column] = 1.0 / f64::from(adj_matrix[row][column]);
            }
        }

        reverse_distance_matrix
    }
}
