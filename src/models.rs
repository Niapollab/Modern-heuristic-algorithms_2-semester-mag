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

struct PairIter<'a> {
    index: usize,
    way: &'a Vec<usize>,
}

impl<'a> Iterator for PairIter<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        let way = self.way;
        let len = way.len();

        if len != 0 && index < len - 1 {
            let edge = (way[index], way[index + 1]);
            self.index += 1;
            Some(edge)
        } else {
            None
        }
    }
}

pub trait WayVecExt {
    #[allow(dead_code)]
    fn iter_edges(&self) -> impl Iterator<Item = (usize, usize)>;
}

impl WayVecExt for Vec<usize> {
    fn iter_edges(&self) -> impl Iterator<Item = (usize, usize)> {
        PairIter {
            index: 0,
            way: &self,
        }
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

    #[allow(dead_code)]
    pub fn contains(&self, from: usize, to: usize) -> bool {
        let way = &self.way;

        for index in 0..way.len() - 1 {
            let (current_from, current_to) = (way[index], way[index + 1]);
            if from == current_from && to == current_to {
                return true;
            }
        }

        false
    }

    fn calculate_score(adj_matrix: &AdjMatrix<u32>, way: &Vec<usize>) -> u64 {
        let sum = way
            .iter_edges()
            .map(|(from, to)| adj_matrix[from][to] as u64)
            .sum();

        sum
    }
}

impl<'a> Display for Way<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let way = &self.way;

        let way_size = way.len();
        let right_arrows_count = way_size - 1;

        let mut parts: Vec<String> = Vec::with_capacity(way_size + right_arrows_count);
        for (from, to) in way.iter_edges() {
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
