// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::prime::{GeneratorTrialDivision, PrimeGenerator};
use euler::Solver;

// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
// Find the sum of all the primes below two million.

pub struct Solver010 {
    pub n: isize
}

impl Default for Solver010 {
    fn default() -> Self {
        Solver010 { n: 2000000 }
    }
}

impl Solver for Solver010 {
    fn solve(&self) -> isize {
        let (mut sum, mut generator) = (0, GeneratorTrialDivision::default());
        loop {
            let prime = generator.next_prime();
            if prime < self.n {
                sum += prime
            } else {
                return sum;
            }
        }
    }
}
