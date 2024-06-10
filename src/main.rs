use cli_utils::{adj_matrix, print_matrix, solver};

mod ant_q_solver;
mod cli_utils;
mod greedy_solver;
mod models;
mod rand_utils;

fn main() {
    const RANDOM_SEED: Option<u64> = None;

    let solver = solver(RANDOM_SEED).unwrap();
    let adj_matrix = adj_matrix(RANDOM_SEED).unwrap();

    println!("Matrix:");
    print_matrix(&adj_matrix);

    let solution = solver.solve(&adj_matrix);
    let score = solution.score();

    println!("Way: {solution}");
    println!("Score: {score}");
}
