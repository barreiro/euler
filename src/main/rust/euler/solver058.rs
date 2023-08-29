// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::cast::Cast;
use algorithm::filter::is_prime;
use algorithm::root::square_u64;
use Solver;

/// Starting with `1` and spiralling anticlockwise in the following way, a square spiral with side length `7` is formed.
/// ```
/// 37 36 35 34 33 32 31
/// 38 17 16 15 14 13 30
/// 39 18  5  4  3 12 29
/// 40 19  6  1  2 11 28
/// 41 20  7  8  9 10 27
/// 42 21 22 23 24 25 26
/// 43 44 45 46 47 48 49
/// ```
/// It is interesting to note that the odd squares lie along the bottom right diagonal, but what is more interesting is that 8 out of the 13 numbers lying along both diagonals are prime; that is, a ratio of `8/13 ≈ 62%`.
///
/// If one complete new layer is wrapped around the spiral above, a square spiral with side length `9` will be formed.
///
/// If this process is continued, what is the side length of the square spiral for which the ratio of primes along both diagonals first falls below `10%`?
pub struct Solver058 {
    pub n: u64,
}

impl Default for Solver058 {
    fn default() -> Self {
        Self { n: 10 }
    }
}

impl Solver for Solver058 {
    fn problem_name(&self) -> &str { "Spiral primes" }

    fn solve(&self) -> i64 {
        (3..).step_by(2).scan(0, |state, level| {
            // test just 3 candidates, as the one in the bottom right diagonal is a perfect square
            *state += (1..4).map(|i| square_u64(level) - i * (level - 1)).filter(is_prime).count().as_u64();
            Some(level).filter(|l| (2 * l - 1) * self.n < *state * 100)
        }).last().map_or(1, |l| l + 2).as_i64()
    }
}
