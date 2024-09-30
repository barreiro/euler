// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use crate::algorithm::cast::Cast;
use crate::algorithm::prime::generator_wheel;
use crate::Solver;

/// By listing the first six prime numbers: `2, 3, 5, 7, 11` and `13`, we can see that the `6`th prime is `13`.
///
/// What is the `10001`st prime number?
pub struct Solver007 {
    pub n: usize
}

impl Default for Solver007 {
    fn default() -> Self {
        Self { n: 10_001 }
    }
}

impl Solver for Solver007 {
    fn problem_name(&self) -> &str { "10001st prime" }

    fn solve(&self) -> i64 {
        generator_wheel().nth(self.n - 1).as_i64()
    }
}
