use crate::models::{Solver, Way};

pub struct GeneticSolver {}

impl Solver for GeneticSolver {
    fn solve<'a>(&self, _: &'a Vec<Vec<u32>>) -> Way<'a> {
        todo!()
    }
}
