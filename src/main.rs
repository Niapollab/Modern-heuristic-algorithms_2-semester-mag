use cli_utils::{adj_matrix, solver};

mod cli_utils;
mod ant_q_solver;
mod greedy_solver;
mod models;
mod rand_utils;

fn main() {
    const RANDOM_SEED: Option<u64> = None;

    let solver = solver(RANDOM_SEED).unwrap();
    let adj_matrix = adj_matrix(RANDOM_SEED).unwrap();

    let solution = solver.solve(&adj_matrix);
    let score = solution.score();

    println!("Way: {solution}");
    println!("Score: {score}");
}
