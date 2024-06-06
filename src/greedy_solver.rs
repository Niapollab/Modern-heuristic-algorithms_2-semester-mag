use crate::models::{Solver, Way};

pub struct GreedySolver {}

impl Solver for GreedySolver {
    fn solve<'a>(&self, adj_matrix: &'a Vec<Vec<u32>>) -> Way<'a> {
        const START_NODE: usize = 0;
        let nodes_count = adj_matrix.len();

        let mut visited = vec![false; nodes_count];
        let mut way: Vec<usize> = Vec::with_capacity(nodes_count);

        let mut node = START_NODE;
        loop {
            visited[node] = true;
            way.push(node);

            let next_node = adj_matrix[node]
                .iter()
                .enumerate()
                .filter(|(index, _)| !visited[*index])
                .min_by_key(|(_, &element)| element)
                .map(|(index, _)| index);

            if let Some(next_node) = next_node {
                node = next_node;
            } else {
                break;
            }
        }
        way.push(START_NODE);

        Way::new(adj_matrix, way)
    }
}
