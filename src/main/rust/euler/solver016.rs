// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::long::{digits_sum, pow_10};
use euler::Solver;

// 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
// What is the sum of the digits of the number 2^1000?

const BASE: isize = 2;
const CELL_THRESHOLD: isize = pow_10(15);

pub struct Solver016 {
    pub n: isize,
}

impl Default for Solver016 {
    fn default() -> Self {
        Solver016 { n: 1000 }
    }
}

impl Solver for Solver016 {
    fn solve(&self) -> isize {
        // each element is a digit. Each iteration we double every digit and adjust
        let mut values = vec![1];

        // since carry never ripples we can iterate backwards, using less memory
        (0..self.n).for_each(|_| (0..values.len()).rev().for_each(|j| {
            values[j] *= BASE;
            if values[j] >= CELL_THRESHOLD {
                // with radix > 2 can use increment and subtract instead of divide and take the remainder
                values[j] -= CELL_THRESHOLD;
                if j == values.len() - 1 { values.push(1) } else { values[j + 1] += 1 }
            }
        }));

        values.iter().map(digits_sum).sum()
    }
}
