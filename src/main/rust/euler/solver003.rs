// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::cast::Cast;
use algorithm::prime::prime_factors;
use Solver;

/// The prime factors of `13195` are `5, 7, 13` and `29`.
///
/// What is the largest prime factor of the number `600851475143`?
pub struct Solver003 {
    pub n: u64
}

impl Default for Solver003 {
    fn default() -> Self {
        Self { n: 600_851_475_143 }
    }
}

impl Solver for Solver003 {
    fn solve(&self) -> i64 {
        prime_factors(self.n).into_keys().max().as_i64()
    }
}
