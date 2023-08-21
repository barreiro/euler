// COPYRIGHT (C) 2023 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::cast::to_i64;
use Solver;

/// Let `r` be the remainder when `(a - 1)^n + (a + 1)^n` is divided by `a^2`.
///
/// For example, if `a = 7` and `n = 3`, then `r = 42` : `6^3 + 8^3 = 728 ≡ 42 mod 49`.
/// And as `n` varies, so too will `r`, but for `a = 7` it turns out that `r_max = 42`.
///
/// For `3 ≤ a ≤ 1000`, find `∑ r_max`.
pub struct Solver120 {
    pub n: u64,
}

impl Default for Solver120 {
    fn default() -> Self {
        Self { n: 1000 }
    }
}

impl Solver for Solver120 {
    fn solve(&self) -> i64 {
        // let r_max = |a| (1..=2 * a).filter(is_odd_u64).map(|n| (n, square_u64(a))).map(|(n, m)| (pow_mod(a - 1, n, m) + pow_mod(a + 1, n, m)) % m).max().expect("There should be a remainder");
        // let r_max = |a| (0..=a).map(|n| (2 * a * (n + 1)) % square_u64(a)).max().expect("There should be a remainder");

        // the expression simplifies to `2an * a^2` and is maximized by the greatest `n < a^2`, that is `n = (a - 1) / 2`, after which the modulo repeats
        (3..=self.n).map(|a| 2 * a * ((a - 1) / 2)).map(to_i64).sum()
    }
}
