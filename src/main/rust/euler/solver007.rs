// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::Solver;
use euler::algorithm::prime::{GeneratorTrialDivision,PrimeGenerator};

// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
// What is the 10001st prime number?

pub struct Solver007 {
    pub n: isize
}

impl Default for Solver007 {
    fn default() -> Self {
        Solver007 { n: 10001 }
    }
}

impl Solver for Solver007 {
    fn solve(&self) -> isize {
        let mut generator = GeneratorTrialDivision::default();
        for _ in 0..self.n - 1 {
            generator.next_prime();
        }
        generator.next_prime()
    }
}
