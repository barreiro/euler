// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::long::{pow_10, to_digits};
use euler::Solver;

// n! means n × (n − 1) × ... × 3 × 2 × 1
// For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800, and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.
// Find the sum of the digits in the number 100!

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
        let (mut factorial, ceiling) = (vec![1], pow_10(10));
        for n in 1..=self.n {
            let mut carry = 0;
            for i in 0..factorial.len() {
                let value = factorial[i] * n + carry;

                // Adjust the buckets that grow beyond the ceiling value, carrying to next bucket
                carry = if value > ceiling { value / ceiling } else { 0 };
                factorial[i] = if value > ceiling { value % ceiling } else { value };
            }
            if carry != 0 {
                // with a small ceiling values would probably need to split the carry into buckets
                factorial.push(carry)
            }
        }
        factorial.iter().fold(0, |sum, &x| sum + to_digits(x).iter().fold(0, |sum, &y| sum + y))
    }
}
