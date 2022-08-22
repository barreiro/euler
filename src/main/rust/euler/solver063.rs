// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::long::{int_log_10, pow_10};
use euler::Solver;

// The 5-digit number, 16807 = 7^5, is also a fifth power. Similarly, the 9-digit number, 134217728 = 8^9, is a ninth power.
//
// How many n-digit positive integers exist which are also an nth power?

pub struct Solver063 {
    pub n: isize
}

impl Default for Solver063 {
    fn default() -> Self {
        Solver063 { n: 21 }
    }
}

impl Solver for Solver063 {
    fn solve(&self) -> isize {
        let pow_digits = |base, exp| {
            let (mut pow, ceiling) = (vec![base], pow_10(15));
            for _ in 2..=exp {
                let mut carry = 0;
                for bucket in &mut pow {
                    let value = *bucket * base + carry;

                    // Adjust the buckets that grow beyond the ceiling value, carrying to next bucket
                    carry = if value > ceiling { value / ceiling } else { 0 };
                    *bucket = if value > ceiling { value % ceiling } else { value };
                }
                if carry != 0 {
                    // with a small ceiling values would probably need to split the carry into buckets
                    pow.push(carry);
                }
            }
            (pow.len() as isize - 1) * 15 + int_log_10(*pow.last().unwrap())
        };

        (1..=self.n).map(|exp| (1..10).map(|base| pow_digits(base, exp)).filter(|&d| d == exp).count() as isize).sum()
    }
}
