// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use crate::algorithm::cast::Cast;
use crate::algorithm::combinatorics::choose;
use crate::Solver;

/// Starting in the top left corner of a `2x2` grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.
///
/// How many such routes are there through a `20x20` grid?
pub struct Solver015 {
    pub n: u64
}

impl Default for Solver015 {
    fn default() -> Self {
        Self { n: 20 }
    }
}

impl Solver for Solver015 {
    fn problem_name(&self) -> &str { "Lattice paths" }

    fn solve(&self) -> i64 {
        choose(2 * self.n, self.n).as_i64()
    }
}
