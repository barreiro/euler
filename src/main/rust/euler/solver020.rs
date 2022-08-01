// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::long::{digits_sum, pow_10};
use euler::Solver;

// n! means n × (n − 1) × ... × 3 × 2 × 1
// For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800, and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.
// Find the sum of the digits in the number 100!

const CELL_THRESHOLD: isize = pow_10(12);

pub struct Solver020 {
    pub n: isize,
}

impl Default for Solver020 {
    fn default() -> Self {
        Solver020 { n: 100 }
    }
}

impl Solver for Solver020 {
    fn solve(&self) -> isize {
        let mut factorial = vec![1];
        (2..=self.n).for_each(|n| {
            let mut carry = None;
            factorial.iter_mut().for_each(|cell| {
                let value = *cell * n + carry.unwrap_or_default();

                // adjust the buckets that grow beyond the ceiling value, carrying to next bucket
                carry = if value > CELL_THRESHOLD { Some(value / CELL_THRESHOLD) } else { None };
                *cell = if value > CELL_THRESHOLD { value % CELL_THRESHOLD } else { value };
            });
            carry.iter().for_each(|&c| factorial.push(c));
        });
        factorial.iter().map(digits_sum).sum()
    }
}
