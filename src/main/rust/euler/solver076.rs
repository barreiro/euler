// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::cast::Cast;
use algorithm::combinatorics::partition;
use Solver;

/// It is possible to write five as a sum in exactly six different ways:
/// `4 + 1`
/// `3 + 2`
/// `3 + 1 + 1`
/// `2 + 2 + 1`
/// `2 + 1 + 1 + 1`
/// `1 + 1 + 1 + 1 + 1`
/// How many different ways can one hundred be written as a sum of at least two positive integers?
pub struct Solver076 {
    pub n: u64,
}

impl Default for Solver076 {
    fn default() -> Self {
        Self { n: 100 }
    }
}

impl Solver for Solver076 {
    fn solve(&self) -> i64 {
        // remove n which is a partition but not a sum
        partition(self.n).as_i64() - 1
    }
}
