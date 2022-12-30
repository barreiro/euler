// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::root::{cube, fourth, square};
use Solver;

/// The sum of the squares of the first ten natural numbers is, `1^2 + 2^2 + ... + 10^2 = 385`
/// The square of the sum of the first ten natural numbers is, `(1 + 2 + ... + 10)^2 = 55^2 = 3025`
/// Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is `3025 − 385 = 2640`.
/// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
pub struct Solver006 {
    pub n: i64,
}

impl Default for Solver006 {
    fn default() -> Self {
        Self { n: 100 }
    }
}

impl Solver for Solver006 {
    fn solve(&self) -> i64 {
        // using Faulhaber's Formula for the square of the sum and Gauss's Formula for the sum of the squares
        let (faulhaber, gauss) = (|n| (fourth(n) + 2 * cube(n) + square(n)) / 4, |n| n * (n + 1) * (2 * n + 1) / 6);
        faulhaber(self.n) - gauss(self.n)
    }
}
