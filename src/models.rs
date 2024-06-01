use std::fmt::Display;

pub struct Way<'a> {
    adj_matrix: &'a Vec<Vec<u32>>,
    way: Vec<usize>,
    score: u64,
}

impl<'a> Way<'a> {
    #[allow(dead_code)]
    pub fn new(adj_matrix: &'a Vec<Vec<u32>>, way: Vec<usize>) -> Self {
        let score = Self::calculate_score(adj_matrix, &way);
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
    pub fn way(&self) -> &Vec<usize> {
        &self.way
    }

    #[allow(dead_code)]
    pub fn score(&self) -> u64 {
        self.score
    }

    fn calculate_score(adj_matrix: &Vec<Vec<u32>>, way: &Vec<usize>) -> u64 {
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

            parts.push(from.to_string());
            parts.push(format!(" -({weight})-> "));
        }
        parts.push(way[right_arrows_count].to_string());

        let serialized = parts.concat();
        write!(f, "{serialized}")
    }
}

pub trait Solver {
    fn solve<'a>(&self, adj_matrix: &'a Vec<Vec<u32>>) -> Way<'a>;
}
