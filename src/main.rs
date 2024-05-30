use cli_utils::{adj_matrix, solver};

mod cli_utils;
mod ant_q_solver;
mod greedy_solver;
mod models;

fn main() {
    let solver = solver().unwrap();
    let adj_matrix = adj_matrix().unwrap();

    let solution = solver.solve(&adj_matrix);
    println!("Solution: {solution}");
}
