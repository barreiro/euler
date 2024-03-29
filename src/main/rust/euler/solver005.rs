// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::long::lcm;
use Solver;

/// `2520` is the smallest number that can be divided by each of the numbers from `1` to `10` without any remainder.
/// What is the smallest positive number that is evenly divisible by all of the numbers from `1` to `20`?
pub struct Solver005 {
    pub n: i64
}

impl Default for Solver005 {
    fn default() -> Self {
        Self { n: 20 }
    }
}

impl Solver for Solver005 {
    fn solve(&self) -> i64 {
        (2..=self.n).fold(1, lcm)
    }
}
