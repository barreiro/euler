// COPYRIGHT (C) 2022 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::cast::Cast;
use algorithm::long::{pow_mod, mul_mod};
use algorithm::root::pow_10;
use Solver;

const FACTOR: u64 = 28_433;
const EXP: u64 = 7_830_457;

/// The first known prime found to exceed one million digits was discovered in 1999, and is a Mersenne prime of the form `2^6972593 − 1`; it contains exactly `2,098,960` digits.
/// Subsequently other Mersenne primes, of the form `2^p − 1`, have been found which contain more digits.
///
/// However, in 2004 there was found a massive non-Mersenne prime which contains `2,357,207` digits: `28433 * 2^7830457 + 1`.
///
/// Find the last ten digits of this prime number.
pub struct Solver097 {
    pub n: u64,
}

impl Default for Solver097 {
    fn default() -> Self {
        Self { n: 10 }
    }
}

impl Solver for Solver097 {
    fn problem_name(&self) -> &str { "Large non-mersenne prime" }

    fn solve(&self) -> i64 {
        mul_mod(FACTOR, pow_mod(2, EXP, pow_10(self.n)), pow_10(self.n)).as_i64() + 1
    }
}
