use std::{
    fs::File,
    io::{stdin, stdout, BufRead, BufReader, Read, Write},
    u32,
};

use rand::Rng;

use crate::{genetic_solver::GeneticSolver, greedy_solver::GreedySolver, models::Solver};

#[derive(Debug)]
pub enum ReadAdjMatrixFileError {
    FileNotFound,
    RowsAndColumnsCountMismatch,
}

#[derive(Debug)]
pub enum ReadAlgorithmError {
    UnknownAlgorithm,
}

#[derive(Debug)]
pub enum ReadAdjMatrixError {
    FileNotFound,
    RowsAndColumnsCountMismatch,
    UnebleToParseUnt32,
    UnknownSource,
}

pub fn solver() -> Result<Box<dyn Solver>, ReadAlgorithmError> {
    let prompt = "Choose algorithm:
1. Greedy algorithm
2. Genetic algorithm
Enter value: ";

    if let Some(option) = choose_option(prompt, 1, 2) {
        match option {
            1 => Ok(Box::new(GreedySolver {})),
            2 => Ok(Box::new(GeneticSolver {})),
            _ => Err(ReadAlgorithmError::UnknownAlgorithm),
        }
    } else {
        Err(ReadAlgorithmError::UnknownAlgorithm)
    }
}

pub fn adj_matrix() -> Result<Vec<Vec<u32>>, ReadAdjMatrixError> {
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

                return match adj_matrix_from_file(path.trim()) {
                    Ok(matrix) => Ok(matrix),
                    Err(ReadAdjMatrixFileError::FileNotFound) => {
                        Err(ReadAdjMatrixError::FileNotFound)
                    }
                    Err(ReadAdjMatrixFileError::RowsAndColumnsCountMismatch) => {
                        Err(ReadAdjMatrixError::RowsAndColumnsCountMismatch)
                    }
                };
            }
            2 => {
                print!("Enter rows count: ");
                stdout().flush().unwrap();

                let mut rows_count = String::new();
                stdin().read_line(&mut rows_count).unwrap();

                let rows_count: usize = match rows_count.trim().parse() {
                    Ok(number) => number,
                    _ => return Err(ReadAdjMatrixError::UnebleToParseUnt32),
                };

                print!("Enter columns count: ");
                stdout().flush().unwrap();

                let mut columns_count = String::new();
                stdin().read_line(&mut columns_count).unwrap();

                let columns_count: usize = match columns_count.trim().parse() {
                    Ok(number) => number,
                    _ => return Err(ReadAdjMatrixError::UnebleToParseUnt32),
                };

                print!("Enter minimum value: ");
                stdout().flush().unwrap();

                let mut min_value = String::new();
                stdin().read_line(&mut min_value).unwrap();

                let min_value: u32 = match min_value.trim().parse() {
                    Ok(number) => number,
                    _ => return Err(ReadAdjMatrixError::UnebleToParseUnt32),
                };

                print!("Enter maximum value: ");
                stdout().flush().unwrap();

                let mut max_value = String::new();
                stdin().read_line(&mut max_value).unwrap();

                let max_value: u32 = match max_value.trim().parse() {
                    Ok(number) => number,
                    _ => return Err(ReadAdjMatrixError::UnebleToParseUnt32),
                };

                Ok(random_adj_matrix(
                    rows_count,
                    columns_count,
                    min_value,
                    max_value,
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
) -> Vec<Vec<u32>> {
    let mut random_provider = rand::thread_rng();
    let mut matrix = vec![vec![0u32; columns_count]; rows_count];

    for row in &mut matrix {
        for element in row {
            *element = random_provider.gen_range(min_value..=max_value);
        }
    }

    matrix
}

fn adj_matrix_from_file(path: &str) -> Result<Vec<Vec<u32>>, ReadAdjMatrixFileError> {
    if let Ok(mut file) = File::open(path) {
        if let Some(matrix) = adj_matrix_from_reader(&mut file) {
            return Ok(matrix);
        }
        return Err(ReadAdjMatrixFileError::RowsAndColumnsCountMismatch);
    }
    Err(ReadAdjMatrixFileError::FileNotFound)
}

fn adj_matrix_from_reader(reader: &mut dyn Read) -> Option<Vec<Vec<u32>>> {
    let buf_reader = BufReader::new(reader);

    let matrix: Vec<Vec<u32>> = buf_reader
        .lines()
        .map(|line| {
            line.unwrap()
                .trim()
                .split_whitespace()
                .flat_map(str::parse)
                .collect()
        })
        .collect();

    if matrix.len() < 1 {
        return Some(matrix);
    }

    let rows_length = matrix.len();
    if matrix.iter().any(|row| row.len() != rows_length) {
        return None;
    }

    Some(matrix)
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
