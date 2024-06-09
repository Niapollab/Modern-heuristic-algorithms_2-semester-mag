use crate::models::{AdjMatrix, Solver, VisitedVecExt, Way};

pub struct GreedySolver {}

impl Solver for GreedySolver {
    fn solve<'a>(&self, adj_matrix: &'a AdjMatrix<u32>) -> Way<'a> {
        const START_NODE: usize = 0;
        let nodes_count = adj_matrix.len();

        let mut visited = vec![false; nodes_count];
        let mut way: Vec<usize> = Vec::with_capacity(nodes_count);

        let mut node = START_NODE;
        loop {
            visited[node] = true;
            way.push(node);

            node = match visited.available_neighbors().min_by(|first, second| {
                let first = adj_matrix[node][*first];
                let second = adj_matrix[node][*second];

                first.cmp(&second)
            }) {
                Some(next_node) => next_node,
                None => break,
            };
        }
        way.push(START_NODE);

        Way::new(adj_matrix, way)
    }
}
