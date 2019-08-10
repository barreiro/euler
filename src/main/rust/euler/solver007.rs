// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::prime::generator_trial_division;
use euler::Solver;

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
        generator_trial_division().nth(self.n as usize - 1).unwrap()
    }
}
