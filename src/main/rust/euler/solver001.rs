// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::long::arithmetic_sum;
use Solver;

// keep sorted
const INPUT_001: &[i64] = &[3, 5];

/// If we list all the natural numbers below 10 that are multiples of `3` or `5`, we get `3, 5, 6` and `9`. The sum of these multiples is `23`.
/// Find the sum of all the multiples of `3` or `5` below `1000`.
pub struct Solver001 {
    pub n: i64,
    pub input: Vec<i64>,
}

impl Default for Solver001 {
    fn default() -> Self {
        Self { n: 1000, input: INPUT_001.to_vec() }
    }
}

impl Solver for Solver001 {
    fn solve(&self) -> i64 {
        // the contribution is the factor multiplied by the sum of the number of occurrences
        let contribution = |factor| factor * arithmetic_sum((self.n - 1) / factor);

        self.input.iter().map(|&factor| contribution(factor) - self.input.iter().take_while(|&&f| f < factor).map(|other| contribution(factor * other)).sum::<i64>()).sum()
    }
}
