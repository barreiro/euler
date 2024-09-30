// COPYRIGHT (C) 2023 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use crate::algorithm::cast::Cast;
use crate::algorithm::prime::radicals_up_to;
use crate::Solver;

/// The radical of `n`, `rad(n)`, is the product of the distinct prime factors of `n`.
///
/// For example, `504 = 2^3 * 3^2 * 7`, so `rad(504) = 2 * 3 * 7 = 42`.
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
    fn problem_name(&self) -> &str { "Ordered radicals" }

    fn solve(&self) -> i64 {
        radicals_up_to(self.ceil.as_u64() + 1).into_iter().enumerate().collect::<Vec<_>>().select_nth_unstable_by_key(self.n, |r| (r.1, r.0)).1.0.as_i64()
    }
}
