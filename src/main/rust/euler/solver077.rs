// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::combinatorics::partition_with_constrains;
use euler::algorithm::long::floor_sqrt;
use euler::algorithm::prime::generator_trial_division;
use euler::Solver;

// It is possible to write ten as the sum of primes in exactly five different ways:
// 7 + 3
// 5 + 5
// 5 + 3 + 2
// 3 + 3 + 2 + 2
// 2 + 2 + 2 + 2 + 2
// What is the first value which can be written as the sum of primes in over five thousand different ways?

pub struct Solver077 {
    pub n: isize
}

impl Default for Solver077 {
    fn default() -> Self {
        Solver077 { n: 5_000 }
    }
}

impl Solver for Solver077 {
    fn solve(&self) -> isize {
        let primes = generator_trial_division().take_while(|&p| p <= (20).max(floor_sqrt(self.n << 1))).collect::<Vec<_>>();
        (3..).find(|&value| partition_with_constrains(value, &primes) - if primes.binary_search(&value).is_ok() { 1 } else { 0 } > self.n).unwrap()
    }
}
