use crate::models::{Solver, Way};

pub struct AntQSolver {}

impl Solver for AntQSolver {
    fn solve<'a>(&self, _: &'a Vec<Vec<u32>>) -> Way<'a> {
        todo!()
    }
}
