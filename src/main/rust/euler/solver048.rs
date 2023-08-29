// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::cast::Cast;
use algorithm::digits::last_digits;
use algorithm::long::pow_mod;
use algorithm::root::pow_10_usize;
use Solver;

const DIGITS: usize = 10;

/// The series, `1^1 + 2^2 + 3^3 + ... + 10^10 = 10405071317`.
///
/// Find the last ten digits of the series, `1^1 + 2^2 + 3^3 + ... + 1000^1000`
pub struct Solver048 {
    pub n: u64,
    pub digits: usize,
}

impl Default for Solver048 {
    fn default() -> Self {
        Self { n: 1000, digits: DIGITS }
    }
}

impl Solver for Solver048 {
    fn solve(&self) -> i64 {
        last_digits((1..=self.n).map(|n| pow_mod(n, n, pow_10_usize(self.digits))).sum(), self.digits).as_i64()
    }
}
