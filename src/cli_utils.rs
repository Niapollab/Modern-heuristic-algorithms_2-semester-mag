use std::{
    fs::File,
    io::{stdin, stdout, BufRead, BufReader, Read, Write},
    u32,
};

use rand::Rng;

use crate::{
    ant_q_solver::{ActionChoiceRule, AntQSolver, DelayedReinforcement},
    greedy_solver::GreedySolver,
    models::{AdjMatrix, Solver},
    rand_utils::random_provider,
};

#[derive(Debug)]
pub enum ReadAlgorithmError {
    UnknownAlgorithm,
    UnableToParseUnt32,
    UnableToParseUsize,
}

#[derive(Debug)]
pub enum ReadAdjMatrixError {
    FileNotFound,
    RowsAndColumnsCountMismatch,
    DiagonalElementsMustBeZero,
    NonDiagonalElementsMustBeGreaterThanZero,
    UnableToParseUnt32,
    UnknownSource,
}

pub fn solver(random_seed: Option<u64>) -> Result<Box<dyn Solver>, ReadAlgorithmError> {
    let prompt = "Choose algorithm:
1. Greedy algorithm
2. Ant-Q algorithm
Enter value: ";

    if let Some(option) = choose_option(prompt, 1, 2) {
        match option {
            1 => Ok(Box::new(GreedySolver {})),
            2 => match build_ant_q_solver(random_seed) {
                Ok(solver) => Ok(Box::new(solver)),
                Err(msg) => Err(msg),
            },
            _ => Err(ReadAlgorithmError::UnknownAlgorithm),
        }
    } else {
        Err(ReadAlgorithmError::UnknownAlgorithm)
    }
}

pub fn build_ant_q_solver(random_seed: Option<u64>) -> Result<AntQSolver, ReadAlgorithmError> {
    print!("Enter max iterations: ");
    stdout().flush().unwrap();

    let mut max_iterations = String::new();
    stdin().read_line(&mut max_iterations).unwrap();

    let max_iterations: u32 = match max_iterations.trim().parse() {
        Ok(number) => number,
        _ => return Err(ReadAlgorithmError::UnableToParseUnt32),
    };

    print!("Enter population size: ");
    stdout().flush().unwrap();

    let mut population_size = String::new();
    stdin().read_line(&mut population_size).unwrap();

    let population_size: usize = match population_size.trim().parse() {
        Ok(number) => number,
        _ => return Err(ReadAlgorithmError::UnableToParseUsize),
    };

    let solver = AntQSolver::new(
        ActionChoiceRule::PseudoRandom {
            q_learning_importance: 1.0,
            heuristic_importance: 2.0,
            initial_q: 0.9,
        },
        DelayedReinforcement::GlobalBest,
        max_iterations,
        population_size,
        random_seed,
        1.0,
        2.0,
        1.0,
        0.1,
        0.1,
        0.3,
    );

    Ok(solver)
}

pub fn adj_matrix(random_seed: Option<u64>) -> Result<AdjMatrix<u32>, ReadAdjMatrixError> {
    let prompt = "Choose matrix source:
1. From file
2. Random
Enter value: ";

    if let Some(option) = choose_option(prompt, 1, 2) {
        return match option {
            1 => {
                print!("Enter path: ");
                stdout().flush().unwrap();

                let mut path = String::new();
                stdin().read_line(&mut path).unwrap();

                adj_matrix_from_file(path.trim())
            }
            2 => {
                print!("Enter rows count: ");
                stdout().flush().unwrap();

                let mut rows_count = String::new();
                stdin().read_line(&mut rows_count).unwrap();

                let rows_count: usize = match rows_count.trim().parse() {
                    Ok(number) => number,
                    _ => return Err(ReadAdjMatrixError::UnableToParseUnt32),
                };

                print!("Enter columns count: ");
                stdout().flush().unwrap();

                let mut columns_count = String::new();
                stdin().read_line(&mut columns_count).unwrap();

                let columns_count: usize = match columns_count.trim().parse() {
                    Ok(number) => number,
                    _ => return Err(ReadAdjMatrixError::UnableToParseUnt32),
                };

                print!("Enter minimum value: ");
                stdout().flush().unwrap();

                let mut min_value = String::new();
                stdin().read_line(&mut min_value).unwrap();

                let min_value: u32 = match min_value.trim().parse() {
                    Ok(number) => number,
                    _ => return Err(ReadAdjMatrixError::UnableToParseUnt32),
                };

                print!("Enter maximum value: ");
                stdout().flush().unwrap();

                let mut max_value = String::new();
                stdin().read_line(&mut max_value).unwrap();

                let max_value: u32 = match max_value.trim().parse() {
                    Ok(number) => number,
                    _ => return Err(ReadAdjMatrixError::UnableToParseUnt32),
                };

                Ok(random_adj_matrix(
                    rows_count,
                    columns_count,
                    min_value,
                    max_value,
                    random_seed,
                ))
            }
            _ => Err(ReadAdjMatrixError::UnknownSource),
        };
    }

    Err(ReadAdjMatrixError::UnknownSource)
}

fn random_adj_matrix(
    rows_count: usize,
    columns_count: usize,
    min_value: u32,
    max_value: u32,
    random_seed: Option<u64>,
) -> AdjMatrix<u32> {
    let mut random_provider = random_provider(random_seed);
    let mut matrix = vec![vec![0u32; columns_count]; rows_count];

    for row in &mut matrix {
        for element in row {
            *element = random_provider.gen_range(min_value..=max_value);
        }
    }

    matrix
}

fn adj_matrix_from_file(path: &str) -> Result<AdjMatrix<u32>, ReadAdjMatrixError> {
    match File::open(path) {
        Ok(mut file) => adj_matrix_from_reader(&mut file),
        _ => Err(ReadAdjMatrixError::FileNotFound),
    }
}

fn adj_matrix_from_reader(reader: &mut dyn Read) -> Result<AdjMatrix<u32>, ReadAdjMatrixError> {
    let buf_reader = BufReader::new(reader);

    let matrix: AdjMatrix<u32> = buf_reader
        .lines()
        .map(|line| {
            line.unwrap()
                .trim()
                .split_whitespace()
                .flat_map(str::parse)
                .collect()
        })
        .collect();

    let size = matrix.len();
    if size == 0 {
        return Ok(matrix);
    }

    for row in 0..size {
        for column in 0..size {
            let element = matrix[row][column];

            if element == 0 && row != column {
                return Err(ReadAdjMatrixError::NonDiagonalElementsMustBeGreaterThanZero);
            }

            if row == column && element != 0 {
                return Err(ReadAdjMatrixError::DiagonalElementsMustBeZero);
            }
        }
    }

    if matrix.iter().any(|row| row.len() != size) {
        return Err(ReadAdjMatrixError::RowsAndColumnsCountMismatch);
    }

    Ok(matrix)
}

fn choose_option(prompt: &str, min_value: u32, max_value: u32) -> Option<u32> {
    print!("{prompt}");
    stdout().flush().unwrap();

    let mut choice = String::new();
    stdin().read_line(&mut choice).unwrap();

    if let Ok(choice) = choice.trim().parse::<u32>() {
        if choice >= min_value && choice <= max_value {
            return Some(choice);
        }
    }

    None
}
