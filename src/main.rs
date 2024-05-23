use petgraph::graphmap::UnGraphMap;
use std::io::{stdin, stdout, Write};

trait Solver {
    fn solve(&self, graph: UnGraphMap<u32, u32>) -> Vec<u32>;
}

struct GreedyAlgorithm {}

impl Solver for GreedyAlgorithm {
    fn solve(&self, _: UnGraphMap<u32, u32>) -> Vec<u32> {
        vec![]
    }
}

struct GeneticAlgorithm {}

impl Solver for GeneticAlgorithm {
    fn solve(&self, _: UnGraphMap<u32, u32>) -> Vec<u32> {
        vec![]
    }
}

#[derive(Debug)]
enum GraphReadError {
    IncorrectFormat,
}

#[derive(Debug)]
enum AlgorithmReadError {
    IncorrectFormat,
}

fn read_graph_from_cli() -> Result<UnGraphMap<u32, u32>, GraphReadError> {
    println!("Enter the graph (from, to, weight):");

    let mut graph = UnGraphMap::<u32, u32>::new();
    loop {
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();

        let numbers = line
            .trim()
            .split_whitespace()
            .flat_map(str::parse::<u32>)
            .collect::<Vec<_>>();

        if numbers.len() < 1 {
            return Ok(graph);
        } else if numbers.len() != 3 {
            return Err(GraphReadError::IncorrectFormat);
        }

        let (from, to, weight) = (numbers[0], numbers[1], numbers[2]);
        graph.add_edge(from, to, weight);
    }
}

fn read_algorithm() -> Result<Box<dyn Solver>, AlgorithmReadError> {
    println!("Choose algorithm:");
    println!("1. Greedy algorithm");
    println!("2. Genetic algorithm");

    print!("Enter value: ");
    stdout().flush().unwrap();

    let mut raw_choice = String::new();
    stdin().read_line(&mut raw_choice).unwrap();

    let parse_result = raw_choice.trim().parse::<u32>();
    if parse_result.is_err() {
        return Err(AlgorithmReadError::IncorrectFormat);
    }

    let choice = parse_result.unwrap() - 1;
    let solver: Result<Box<dyn Solver>, _> = match choice {
        0 => Ok(Box::new(GreedyAlgorithm {})),
        1 => Ok(Box::new(GeneticAlgorithm {})),
        _ => Err(AlgorithmReadError::IncorrectFormat),
    };

    return solver;
}

fn build_way_chain(way: &Vec<u32>) -> String {
    let segments = way.iter().map(ToString::to_string).collect::<Vec<_>>();
    segments.join(" -> ")
}

fn main() {
    let algorithm = read_algorithm().expect("Unable to read the algorithm");
    let graph = read_graph_from_cli().expect("Unable to read the graph");

    let solution = algorithm.solve(graph);

    let way = build_way_chain(&solution);
    println!("{way}");
}
