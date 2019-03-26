// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::combinatorics::choose;
use euler::Solver;

// Starting in the top left corner of a 2x2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.
// How many such routes are there through a 20x20 grid?

pub struct Solver015 {
    pub n: isize
}

impl Default for Solver015 {
    fn default() -> Self {
        Solver015 { n: 20 }
    }
}

impl Solver for Solver015 {
    fn solve(&self) -> isize {
        choose(2 * self.n, self.n)
    }
}
