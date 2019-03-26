// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::long::arithmetic_sum;
use euler::Solver;

// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
// Find the sum of all the multiples of 3 or 5 below 1000.

const INPUT_001: &[isize] = &[3, 5];

pub struct Solver001<'a> {
    pub n: isize,
    pub input: &'a [isize],
}

impl<'a> Default for Solver001<'a> {
    fn default() -> Self {
        Solver001 { n: 1000, input: INPUT_001 }
    }
}

impl<'a> Solver for Solver001<'a> {
    fn solve(&self) -> isize {
        let mut values = vec![];

        for factor in self.input {
            values.push(contribution(*factor, self.n - 1));

            for other in self.input {
                if other < factor {
                    values.push(-contribution(*factor * *other, self.n - 1));
                }
            }
        }
        values.iter().sum()
    }
}

fn contribution(factor: isize, ceiling: isize) -> isize {
    // the sum is the factor multiplied by the sum of the number of occurrences
    factor * arithmetic_sum(ceiling / factor)
}
