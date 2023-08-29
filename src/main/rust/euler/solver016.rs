// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::digits::digits_sum;
use algorithm::root::pow_10;
use Solver;

const DEFAULT_BASE: u64 = 2;
const CELL_THRESHOLD: u64 = pow_10(15);

/// `2^15 = 32768` and the sum of its digits is `3 + 2 + 7 + 6 + 8 = 26`.
///
/// What is the sum of the digits of the number `2^1000`?
pub struct Solver016 {
    pub n: u64,
    pub base: u64,
}

impl Default for Solver016 {
    fn default() -> Self {
        Self { n: 1000, base: DEFAULT_BASE }
    }
}

impl Solver for Solver016 {
    fn problem_name(&self) -> &str { "Power digit sum" }

    fn solve(&self) -> i64 {
        // each element is a digit. Each iteration we double every digit and adjust
        let mut values = vec![1];

        // since carry never ripples we can iterate backwards, using less memory
        (0..self.n).for_each(|_| (0..values.len()).rev().for_each(|j| {
            values[j] *= self.n;
            if values[j] >= CELL_THRESHOLD {
                // with radix > 2 can use increment and subtract instead of divide and take the remainder
                values[j] -= CELL_THRESHOLD;
                if j == values.len() - 1 { values.push(1) } else { values[j + 1] += 1 }
            }
        }));

        values.into_iter().map(digits_sum).sum()
    }
}
