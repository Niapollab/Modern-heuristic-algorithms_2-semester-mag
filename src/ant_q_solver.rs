use rand::{distributions::Standard, rngs::StdRng, Rng};

use crate::{
    models::{AdjMatrix, Solver, VisitedVecExt, Way, WayVecExt},
    rand_utils::{random_provider, RngDistributionExt},
};

struct AntState {
    visited: Vec<bool>,
    way: Vec<usize>,
}

impl AntState {
    #[inline]
    fn unchecked_visit(&mut self, node: usize) {
        self.visited[node] = true;
        self.way.push(node);
    }

    #[inline]
    fn start_node(&self) -> usize {
        self.way[0]
    }

    #[inline]
    fn last_node(&self) -> usize {
        let way = &self.way;
        let size = way.len();

        way[size - 1]
    }

    #[inline]
    fn last_edge(&self) -> (usize, usize) {
        let way = &self.way;
        let size = way.len();

        (way[size - 2], way[size - 1])
    }
}

#[derive(Copy, Clone)]
pub enum ActionChoiceRule {
    PseudoRandom {
        q_learning_importance: f64,
        heuristic_importance: f64,
        initial_q: f64,
    },
    PseudoRandomProportional {
        q_learning_importance: f64,
        heuristic_importance: f64,
    },
    RandomProportional {
        q_learning_importance: f64,
        heuristic_importance: f64,
    },
}

#[derive(Copy, Clone)]
pub enum DelayedReinforcement {
    GlobalBest,
    IterationBest,
    AntSystem,
}

struct AlgorithmState<'a> {
    adj_matrix: &'a AdjMatrix<u32>,
    reverse_distance_matrix: AdjMatrix<f64>,
    action_choice_rule: ActionChoiceRule,
    delayed_reinforcement: DelayedReinforcement,

    max_iteration: u32,
    population_size: usize,
    random_provider: StdRng,

    pheromone_importance: f64,
    destination_importance: f64,
    pheromone_intensity: f64,
    pheromone_evaporation: f64,
    current_learning_speed: f64,
    next_learning_speed: f64,
    next_action_importance: f64,

    iteration: u32,
    pheromone_matrix: AdjMatrix<f64>,
    heuristic_matrix: AdjMatrix<f64>,
    q_learning_matrix: AdjMatrix<f64>,
    best_way: Option<Way<'a>>,
}

impl<'a> Iterator for AlgorithmState<'a> {
    type Item = Way<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iteration >= self.max_iteration {
            return None;
        }
        self.iteration += 1;

        self.recalculate_heuristic_matrix();
        let population = self.build_population();
        self.spread_pheromone(&population);

        let iteration_best_way_index = population
            .iter()
            .enumerate()
            .min_by_key(|(_, way)| *way)
            .map(|(index, _)| index)
            .unwrap();

        let iteration_best_way = &population[iteration_best_way_index];
        let global_best_way = match &self.best_way {
            Some(way) if iteration_best_way < way => iteration_best_way,
            Some(way) => way,
            None => iteration_best_way,
        };

        let delta_q_learing_matrix =
            self.delta_q_learing_matrix(&population, iteration_best_way, global_best_way);
        self.apply_delayed_reinforcement(&population, &delta_q_learing_matrix);

        let iteration_best_way = population
            .into_iter()
            .nth(iteration_best_way_index)
            .unwrap();
        let global_best_way = self.global_best_way(iteration_best_way);

        Some(global_best_way)
    }
}

impl<'a> AlgorithmState<'a> {
    #[inline]
    fn recalculate_heuristic_matrix(&mut self) {
        let nodes_count = self.adj_matrix.len();

        for row in 0..nodes_count {
            for column in 0..nodes_count {
                let first = self.pheromone_matrix[row][column].powf(self.pheromone_importance);
                let second =
                    self.reverse_distance_matrix[row][column].powf(self.destination_importance);

                self.heuristic_matrix[row][column] = first * second;
            }
        }
    }

    #[inline]
    fn build_population(&mut self) -> Vec<Way<'a>> {
        let adj_matrix = self.adj_matrix;
        let nodes_count = adj_matrix.len();

        let population_size = self.population_size;
        let mut ants: Vec<AntState> = Vec::with_capacity(population_size);

        for _ in 0..population_size {
            let start_node = self.random_provider.gen_range(0..nodes_count);
            let mut ant = AntState {
                visited: vec![false; nodes_count],
                way: Vec::with_capacity(nodes_count),
            };

            ant.unchecked_visit(start_node);
            ants.push(ant);
        }

        loop {
            let is_all_built = ants.iter().all(|ant| ant.visited.iter().all(|node| *node));
            if is_all_built {
                break;
            }

            for ant in &mut ants {
                if let Some(next_node) = self.find_next(ant) {
                    ant.unchecked_visit(next_node);
                }
            }

            self.recalculate_q_learning_matrix(&ants);
        }

        for ant in &mut ants {
            ant.unchecked_visit(ant.start_node());
        }
        self.recalculate_q_learning_matrix(&ants);

        let ant_ways: Vec<Way<'a>> = ants
            .into_iter()
            .map(|ant| Way::new(adj_matrix, ant.way))
            .collect();

        ant_ways
    }

    #[inline]
    fn find_next(&mut self, ant: &mut AntState) -> Option<usize> {
        match self.action_choice_rule {
            ActionChoiceRule::PseudoRandom {
                q_learning_importance,
                heuristic_importance,
                initial_q,
            } => self.pseudo_random(ant, q_learning_importance, heuristic_importance, initial_q),
            ActionChoiceRule::PseudoRandomProportional {
                q_learning_importance,
                heuristic_importance,
            } => self.pseudo_random_proportional(ant, q_learning_importance, heuristic_importance),
            ActionChoiceRule::RandomProportional {
                q_learning_importance,
                heuristic_importance,
            } => self.random_proportional(ant, q_learning_importance, heuristic_importance),
        }
    }

    #[inline]
    fn pseudo_random(
        &mut self,
        ant: &mut AntState,
        q_learning_importance: f64,
        heuristic_importance: f64,
        initial_q: f64,
    ) -> Option<usize> {
        let random_provider = &mut self.random_provider;

        let adj_matrix = self.adj_matrix;
        let nodes_count = adj_matrix.len();

        let q: f64 = random_provider.sample(Standard);
        let from_node = ant.last_node();

        let next_node = if q <= initial_q {
            let node = ant
                .visited
                .available_neighbors()
                .map(|index| {
                    let first =
                        self.q_learning_matrix[from_node][index].powf(q_learning_importance);
                    let second = self.heuristic_matrix[from_node][index].powf(heuristic_importance);

                    let probability = first * second;
                    (index, probability)
                })
                .max_by(|(_, first), (_, second)| first.partial_cmp(second).unwrap())
                .map(|(index, _)| index);

            node
        } else {
            let random_node = random_provider.gen_range(0..nodes_count);
            Some(random_node)
        };

        next_node
    }

    #[inline]
    fn pseudo_random_proportional(
        &mut self,
        _ant: &mut AntState,
        _q_learning_importance: f64,
        _heuristic_importance: f64,
    ) -> Option<usize> {
        todo!()
    }

    #[inline]
    fn random_proportional(
        &mut self,
        _ant: &mut AntState,
        _q_learning_importance: f64,
        _heuristic_importance: f64,
    ) -> Option<usize> {
        todo!()
    }

    #[inline]
    fn recalculate_q_learning_matrix(&mut self, ants: &Vec<AntState>) {
        const MIN_Q_VALUE: f64 = 1e-5;
        let q_learning_matrix = &mut self.q_learning_matrix;

        for ant in ants {
            let (from, to) = ant.last_edge();

            let future_q = match ant
                .visited
                .available_neighbors()
                .map(|node| q_learning_matrix[to][node])
                .max_by(|f, s| f.partial_cmp(s).unwrap())
            {
                Some(value) => value,
                None => 0.0,
            };

            let q_value = &mut q_learning_matrix[from][to];
            *q_value = self.current_learning_speed * *q_value
                + self.next_learning_speed * self.next_action_importance * future_q;

            if *q_value < MIN_Q_VALUE {
                *q_value = MIN_Q_VALUE;
            }
        }
    }

    #[inline]
    fn delta_q_learing_matrix(
        &self,
        population: &Vec<Way<'a>>,
        iteration_best_way: &Way<'a>,
        global_best_way: &Way<'a>,
    ) -> AdjMatrix<f64> {
        let nodes_count = self.adj_matrix.len();
        let mut delta_q_learing_matrix = vec![vec![0.0; nodes_count]; nodes_count];

        match self.delayed_reinforcement {
            DelayedReinforcement::GlobalBest | DelayedReinforcement::IterationBest => {
                let best_way = if let DelayedReinforcement::GlobalBest = self.delayed_reinforcement
                {
                    global_best_way
                } else {
                    iteration_best_way
                };

                let way = best_way.way();
                let score = best_way.score() as f64;

                for (from, to) in way.iter_edges() {
                    delta_q_learing_matrix[from][to] = 10.0 / score;
                }
            }
            DelayedReinforcement::AntSystem => {
                for ant in population {
                    let way = ant.way();
                    let score = ant.score() as f64;

                    for (from, to) in way.iter_edges() {
                        delta_q_learing_matrix[from][to] += 10.0 / score;
                    }
                }
            }
        };

        delta_q_learing_matrix
    }

    #[inline]
    fn apply_delayed_reinforcement(
        &mut self,
        population: &Vec<Way<'a>>,
        delta_q_learing_matrix: &AdjMatrix<f64>,
    ) {
        const MIN_Q_VALUE: f64 = 1e-5;
        let nodes_count = self.adj_matrix.len();

        match self.delayed_reinforcement {
            DelayedReinforcement::GlobalBest | DelayedReinforcement::IterationBest => {
                for ant in population {
                    for (from, to) in ant.way().iter_edges() {
                        let q_value = &mut self.reverse_distance_matrix[from][to];
                        let delta = delta_q_learing_matrix[from][to];

                        *q_value = self.current_learning_speed * *q_value
                            + self.next_learning_speed * delta;

                        if *q_value < MIN_Q_VALUE {
                            *q_value = MIN_Q_VALUE;
                        }
                    }
                }
            }
            DelayedReinforcement::AntSystem => {
                for row in 0..nodes_count {
                    for column in 0..nodes_count {
                        let q_value = &mut self.reverse_distance_matrix[row][column];
                        let delta = delta_q_learing_matrix[row][column];

                        *q_value = self.current_learning_speed * *q_value + delta;

                        if *q_value < MIN_Q_VALUE {
                            *q_value = MIN_Q_VALUE;
                        }
                    }
                }
            }
        }
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
    action_choice_rule: ActionChoiceRule,
    delayed_reinforcement: DelayedReinforcement,
    max_iteration: u32,
    population_size: usize,
    random_seed: Option<u64>,
    pheromone_importance: f64,
    destination_importance: f64,
    pheromone_intensity: f64,
    pheromone_evaporation: f64,
    learning_speed: f64,
    next_action_importance: f64,
}

impl AntQSolver {
    pub fn new(
        action_choice_rule: ActionChoiceRule,
        delayed_reinforcement: DelayedReinforcement,
        max_iteration: u32,
        population_size: usize,
        random_seed: Option<u64>,
        pheromone_importance: f64,
        destination_importance: f64,
        pheromone_intensity: f64,
        pheromone_evaporation: f64,
        learning_speed: f64,
        next_action_importance: f64,
    ) -> Self {
        Self {
            action_choice_rule,
            delayed_reinforcement,
            max_iteration,
            population_size,
            random_seed,
            pheromone_importance,
            destination_importance,
            pheromone_intensity,
            pheromone_evaporation,
            learning_speed,
            next_action_importance,
        }
    }
}

impl Solver for AntQSolver {
    fn solve<'a>(&self, adj_matrix: &'a AdjMatrix<u32>) -> Way<'a> {
        const PHEROMONE_INIT_STATE: f64 = 1.0;
        let nodes_count = adj_matrix.len();

        let reverse_distance_matrix = AntQSolver::reverse_distance_matrix(adj_matrix);
        let q_learning_matrix = AntQSolver::q_learning_matrix(adj_matrix);

        let state = AlgorithmState {
            adj_matrix,
            reverse_distance_matrix,
            action_choice_rule: self.action_choice_rule,
            delayed_reinforcement: self.delayed_reinforcement,
            random_provider: random_provider(self.random_seed),
            max_iteration: self.max_iteration,
            population_size: self.population_size,
            pheromone_importance: self.pheromone_importance,
            destination_importance: self.destination_importance,
            pheromone_intensity: self.pheromone_intensity,
            pheromone_evaporation: self.pheromone_evaporation,
            current_learning_speed: 1.0 - self.learning_speed,
            next_learning_speed: self.learning_speed,
            next_action_importance: self.next_action_importance,
            iteration: 0,
            pheromone_matrix: vec![vec![PHEROMONE_INIT_STATE; nodes_count]; nodes_count],
            heuristic_matrix: vec![vec![0.0; nodes_count]; nodes_count],
            q_learning_matrix,
            best_way: None,
        };

        state.last().unwrap()
    }
}

impl AntQSolver {
    #[inline]
    fn reverse_distance_matrix(adj_matrix: &AdjMatrix<u32>) -> AdjMatrix<f64> {
        let nodes_count = adj_matrix.len();
        let mut reverse_distance_matrix = vec![vec![0.0; nodes_count]; nodes_count];

        for row in 0..nodes_count {
            for column in 0..nodes_count {
                reverse_distance_matrix[row][column] = 1.0 / adj_matrix[row][column] as f64;
            }
        }

        reverse_distance_matrix
    }

    #[inline]
    fn q_learning_matrix(adj_matrix: &AdjMatrix<u32>) -> AdjMatrix<f64> {
        let nodes_count = adj_matrix.len();
        let edges_count = (nodes_count * (nodes_count - 1)) as f64;

        let average_weight: f64 = adj_matrix
            .iter()
            .flat_map(|row| row)
            .map(|weight| *weight as f64)
            .sum();
        let average_weight = 1.0 / (average_weight * edges_count);

        let q_learning_matrix = vec![vec![average_weight; nodes_count]; nodes_count];
        q_learning_matrix
    }
}
