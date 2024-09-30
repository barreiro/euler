// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use crate::algorithm::cast::{Cast, to_i64, to_u64};
use crate::algorithm::filter::{greater_than_u64, is_odd, is_prime};
use crate::algorithm::prime::descending_primes;
use crate::algorithm::root::square;
use crate::Solver;

const HEEGNER: i64 = -163;

/// Euler discovered the remarkable quadratic formula: `n^2 + n + 41`
///
/// It turns out that the formula will produce `40` primes for the consecutive values `n = 0` to `39`.
///
/// However, when `n = 40`, `402 + 40 + 41 = 40(40 + 1) + 41` is divisible by `41`, and certainly when `n = 41, 41^2 + 41 + 41` is clearly divisible by `41`.
///
/// The incredible formula `n^2 − 79*n + 1601` was discovered, which produces `80` primes for the consecutive values `n = 0` to `79`.
///
/// The product of the coefficients, `−79` and `1601`, is `−126479`.
///
/// Considering quadratics of the form: `n^2 + a*n + b`, where `|a| < 1000` and `|b| < 1000` where `|n|` is the modulus/absolute value of n e.g. `|11| = 11` and `|−4| = 4`
///
/// Find the product of the coefficients, `a` and `b`, for the quadratic expression that produces the maximum number of primes for consecutive values of `n`, starting with `n = 0`.
pub struct Solver027 {
    pub n: u64,
}

impl Default for Solver027 {
    fn default() -> Self {
        Self { n: 1000 }
    }
}

impl Solver for Solver027 {
    fn problem_name(&self) -> &str { "Quadratic primes" }

    #[allow(clippy::maybe_infinite_iter)]
    fn solve(&self) -> i64 {
        // conjecture: `a` is odd negative and `b` is one of the 10% highest primes
        // the discriminant must be a Heegner number, in particular -163
        let primes = descending_primes(self.n).take_while(greater_than_u64(self.n - self.n / 10)).map(to_i64).collect::<Vec<_>>();
        let primes_contains_b = |(_, b): &(_, i64)| primes.binary_search_by(|other| b.cmp(other)).is_ok(); // reverse compare as primes are descending
        let prime_count = |(a, b): &(i64, i64)| (0..).map(|n| square(n) + a * n + b).map(to_u64).take_while(is_prime).count();

        (-self.n.as_i64()..0).filter(is_odd).map(|a| (a, (square(a) - HEEGNER) / 4)).filter(primes_contains_b).max_by_key(prime_count).map(|(a, b)| a * b).as_i64()
    }
}
