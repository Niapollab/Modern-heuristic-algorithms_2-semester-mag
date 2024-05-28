use std::fmt::Display;

pub struct Way<'a> {
    adj_matrix: &'a Vec<Vec<u32>>,
    way: Vec<u32>,
    score: u64,
}

impl<'a> Way<'a> {
    #[allow(dead_code)]
    pub fn new(adj_matrix: &'a Vec<Vec<u32>>, way: Vec<u32>, score: u64) -> Self {
        Way {
            adj_matrix,
            way,
            score,
        }
    }

    #[allow(dead_code)]
    pub fn adj_matrix(&self) -> &Vec<Vec<u32>> {
        &self.adj_matrix
    }

    #[allow(dead_code)]
    pub fn way(&self) -> &Vec<u32> {
        &self.way
    }

    #[allow(dead_code)]
    pub fn score(&self) -> u64 {
        self.score
    }
}

impl<'a> Display for Way<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let serialized = self
            .way
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" -> ");

        write!(f, "{serialized}")
    }
}

pub trait Solver {
    fn solve<'a>(&self, adj_matrix: &'a Vec<Vec<u32>>) -> Way<'a>;
}
