// COPYRIGHT (C) 2023 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::iter::once;

use algorithm::cast::Cast;
use Solver;

/// The radical of `n`, `rad(n)`, is the product of the distinct prime factors of `n`.
///
/// For example, `504 = 2^3 * 3^2 * 7`, so `rad(504) = 2 * 3 *7 = 42`.
///
/// If we calculate `rad(n)` for `1 ≤ n ≤ 10`, then sort them on `rad(n)`, and sorting on `n` if the radical values are equal, we get:
///```
///  Unsorted      Sorted
///  n rad(n)    n rad(n)    k
///  1      1    1      1    1
///  2      2    2      2    2
///  3      3    4      2    3
///  4      2    8      2    4
///  5      5    3      3    5
///  6      6    9      3    6
///  7      7    5      5    7
///  8      2    6      6    8
///  9      3    7      7    9
/// 10     10   10     10   10
///```
/// Let `E(k)` be the `kth` element in the sorted `n` column; for example, `E(4) = 8` and `E(6)=9`.
///
/// If `rad(n)` is sorted for `1 ≤ n ≤ 100000`, find `E(10000)`.
pub struct Solver124 {
    pub n: usize,
    pub ceil: usize,
}

impl Default for Solver124 {
    fn default() -> Self {
        Self { n: 10_000, ceil: 100_000 }
    }
}

impl Solver for Solver124 {
    fn solve(&self) -> i64 {
        // let primes = primes_up_to(ceil_sqrt_u64(self.ceil)).collect::<Vec<_>>();
        // let mut radicals = (1..=self.ceil).map(|n| (prime_factors_with_cache(n, &primes).keys().product::<u64>(), n)).collect::<Vec<_>>();
        // radicals.select_nth_unstable(self.n.as_usize() - 1).1.1.as_i64()

        // faster way to calculate radicals by sieving instead of multiplying the prime factors
        let mut radicals = (once(1).cycle()).zip(0u64..).take(self.ceil + 1).collect::<Vec<_>>();
        (2..radicals.len()).for_each(|p| if radicals[p].0 == 1 { (p..=self.ceil).step_by(p).for_each(|m| radicals[m].0 *= p) });
        radicals.select_nth_unstable(self.n).1.1.as_i64()
    }
}
