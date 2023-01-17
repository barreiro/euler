// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::convert::identity;

use algorithm::cast::Cast;
use algorithm::digits::is_permutation;
use algorithm::prime::primes_up_to;
use algorithm::root::{floor_sqrt_u64, pow_10};
use Solver;

/// Euler's Totient function, `φ(n)` (sometimes called the phi function), is used to determine the number of positive numbers less than or equal to `n` which are relatively prime to `n`.
/// For example, as `1, 2, 4, 5, 7, and 8`, are all less than nine and relatively prime to nine, `φ(9)=6`.
/// The number `1` is considered to be relatively prime to every positive number, so `φ(1)=1`.
///
/// Interestingly, `φ(87109) = 79180`, and it can be seen that `87109` is a permutation of `79180`.
/// Find the value of `n`, `1 < n < 10^7`, for which `φ(n)` is a permutation of `n` and the ratio `n/φ(n)` produces a minimum.
pub struct Solver070 {
    pub n: u64,
}

impl Default for Solver070 {
    fn default() -> Self {
        Self { n: 7 }
    }
}

impl Solver for Solver070 {
    fn solve(&self) -> i64 {
        let (domain, primes) = (pow_10(self.n), primes_up_to(10 * floor_sqrt_u64(pow_10(self.n))).collect::<Vec<_>>());
        let primes_index = |value| primes.binary_search(&value).unwrap_or_else(identity);

        // generates products from 2 prime factors, starting with the biggest, as this minimizes the phi function
        // optimization: assume that the absolute minimum is amongst the first self.n solutions found
        (1..primes_index(floor_sqrt_u64(domain))).rev().filter_map(|small|
            (small..primes_index(domain / primes[small])).rev().find_map(|big| {
                let product = primes[big] * primes[small];
                let phi = product - primes[big] - primes[small] + 1;
                is_permutation(product, phi).then_some((domain * product / phi, product))
            })).take(self.n.as_usize()).min().map(|(_, n)| n).as_i64()
    }
}
