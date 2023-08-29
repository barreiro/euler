// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::cast::to_i64;
use algorithm::prime::primes_up_to;
use Solver;

/// The sum of the primes below 10 is `2 + 3 + 5 + 7 = 17`.
///
/// Find the sum of all the primes below two million.
pub struct Solver010 {
    pub n: u64,
}

impl Default for Solver010 {
    fn default() -> Self {
        Self { n: 2_000_000 }
    }
}

impl Solver for Solver010 {
    fn problem_name(&self) -> &str { "Summation of primes" }

    fn solve(&self) -> i64 {
        primes_up_to(self.n).map(to_i64).sum()
    }
}
