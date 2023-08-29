// COPYRIGHT (C) 2023 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::cast::Cast;
use algorithm::prime::generator_wheel;
use algorithm::root::pow_10;
use Solver;

/// Let `p_n` be the `n`th prime: `2, 3, 5, 7, 11, ...`, and let `r` be the remainder when `(p_n - 1)^n + (p_n + 1)^n` is divided by `p_n^2`.
///
/// For example, when `n = 3`, `p_3 = 5`, and `4^3 + 6^3 = 280 ≡ 5 mod 25`.
///
/// The least value of `n` for which the remainder first exceeds `10^9` is `7037`.
///
/// Find the least value of `n` for which the remainder first exceeds `10^10`.
pub struct Solver123 {
    pub n: u64,
}

impl Default for Solver123 {
    fn default() -> Self {
        Self { n: 10 }
    }
}

impl Solver for Solver123 {
    fn problem_name(&self) -> &str { "Prime square remainders" }

    fn solve(&self) -> i64 {
        // the expression simplifies to `2 * n * p_n` when `n` is odd (otherwise result is `2`)
        generator_wheel().zip(1..).find(|&(p, n)| n & 1 != 0 && 2 * n * p > pow_10(self.n)).map(|(_, n)| n).as_i64()
    }
}
