// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::long::pow_10;
use euler::algorithm::prime::{generator_trial_division, is_prime};
use euler::Solver;

// The prime 41, can be written as the sum of six consecutive primes: 41 = 2 + 3 + 5 + 7 + 11 + 13
// This is the longest sum of consecutive primes that adds to a prime below one-hundred.
// The longest sum of consecutive primes below one-thousand that adds to a prime, contains 21 terms, and is equal to 953.
//
// Which prime, below one-million, can be written as the sum of the most consecutive primes?

pub struct Solver050 {
    pub n: isize,
}

impl Default for Solver050 {
    fn default() -> Self {
        Solver050 { n: 6 }
    }
}

impl Solver for Solver050 {
    fn solve(&self) -> isize {
        // the list of primes which sum is below the limit, then starting on the greater window sizes try to find a sum that is prime
        let primes: Vec<_> = generator_trial_division().scan(0, |acc, p| {
            *acc += p;
            Some(p).filter(|_| *acc < pow_10(self.n))
        }).collect();

        (1..=primes.len()).rev().find_map(|len| primes.windows(len).rev().map(|arr| arr.iter().sum()).find(is_prime)).unwrap()
    }
}
