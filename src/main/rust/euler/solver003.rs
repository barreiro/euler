// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::prime::prime_factors;
use euler::Solver;

// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143 ?

pub struct Solver003 {
    pub n: isize
}

impl Default for Solver003 {
    fn default() -> Self {
        Solver003 { n: 600_851_475_143 }
    }
}

impl Solver for Solver003 {
    fn solve(&self) -> isize {
        prime_factors(self.n).into_keys().max().unwrap()
    }
}
