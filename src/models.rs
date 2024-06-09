use std::{cmp::Ordering, fmt::Display};

pub type AdjMatrix<T> = Vec<Vec<T>>;

pub trait VisitedVecExt {
    #[allow(dead_code)]
    fn available_neighbors(&self) -> impl Iterator<Item = usize>;
}

impl VisitedVecExt for Vec<bool> {
    fn available_neighbors(&self) -> impl Iterator<Item = usize> {
        let nodes_count = self.len();
        (0..nodes_count).into_iter().filter(|index| !self[*index])
    }
}

pub struct Way<'a> {
    adj_matrix: &'a AdjMatrix<u32>,
    way: Vec<usize>,
    score: u64,
}

impl<'a> Way<'a> {
    #[allow(dead_code)]
    pub fn new(adj_matrix: &'a AdjMatrix<u32>, way: Vec<usize>) -> Self {
        let score = Self::calculate_score(adj_matrix, &way);
        Self {
            adj_matrix,
            way,
            score,
        }
    }

    #[allow(dead_code)]
    pub fn adj_matrix(&self) -> &AdjMatrix<u32> {
        &self.adj_matrix
    }

    #[allow(dead_code)]
    pub fn way(&self) -> &Vec<usize> {
        &self.way
    }

    #[allow(dead_code)]
    pub fn score(&self) -> u64 {
        self.score
    }

    fn calculate_score(adj_matrix: &AdjMatrix<u32>, way: &Vec<usize>) -> u64 {
        let mut sum = 0u64;

        for index in 0..way.len() - 1 {
            let (from, to) = (way[index], way[index + 1]);
            sum += u64::from(adj_matrix[from][to]);
        }

        sum
    }
}

impl<'a> Display for Way<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let way = &self.way;

        let way_size = way.len();
        let right_arrows_count = way_size - 1;

        let mut parts: Vec<String> = Vec::with_capacity(way_size + right_arrows_count);
        for index in 0..right_arrows_count {
            let (from, to) = (way[index], way[index + 1]);
            let weight = self.adj_matrix[from][to];

            parts.push((from + 1).to_string());
            parts.push(format!(" -({weight})-> "));
        }
        parts.push((way[right_arrows_count] + 1).to_string());

        let serialized = parts.concat();
        write!(f, "{serialized}")
    }
}

impl<'a> Clone for Way<'a> {
    fn clone(&self) -> Self {
        Self {
            adj_matrix: self.adj_matrix,
            way: self.way.clone(),
            score: self.score,
        }
    }
}

impl<'a> Ord for Way<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl<'a> PartialOrd for Way<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl<'a> PartialEq for Way<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.adj_matrix == other.adj_matrix && self.way == other.way && self.score == other.score
    }
}

impl<'a> Eq for Way<'a> {}

pub trait Solver {
    fn solve<'a>(&self, adj_matrix: &'a AdjMatrix<u32>) -> Way<'a>;
}
