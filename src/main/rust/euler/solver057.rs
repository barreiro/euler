// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::iter::from_fn;

use algorithm::cast::Cast;
use algorithm::root::{int_log_10, pow_10};
use Solver;

// truncate the continuous fraction to this value (16 digits prevents overflows)
const THRESHOLD: u64 = pow_10(16);

/// It is possible to show that the square root of two can be expressed as an infinite continued fraction.
/// ```
/// sqrt(2) = 1 + 1 / (2 + 1 / (2 + 1 / (2 + ... )
/// ```
/// By expanding this for the first four iterations, we get:
/// ```
/// 1 + (1 / 2) = 3/2 = 1.5
/// 1 + (1 / (2 + 1 / 2) = 7/5 = 1.4
/// 1 + (1 / (2 + 1 / (2 + 1 / 2) = 17/12 = 1.41666...
/// 1 + (1 / (2 + 1 / (2 + 1 / (2 + 1 / 2) = 41/29 = 1.41379...
/// ```
/// The next three expansions are `99/70`, `239/169`, and `577/408`, but the eighth expansion, `1393/985`, is the first example where the number of digits in the numerator exceeds the number of digits in the denominator.
///
/// In the first one-thousand expansions, how many fractions contain a numerator with more digits than the denominator?
pub struct Solver057 {
    pub n: usize,
}

impl Default for Solver057 {
    fn default() -> Self {
        Self { n: 1000 }
    }
}

impl Solver for Solver057 {
    fn problem_name(&self) -> &str { "Square root convergents" }

    fn solve(&self) -> i64 {
        continued_sqroot_two().take(self.n).filter(|&(n, d)| int_log_10(n) > int_log_10(d)).count().as_i64()
    }
}

// --- //

/// iterator for continued fractions that approach `sqrt(2)`
fn continued_sqroot_two() -> impl Iterator<Item=(u64, u64)> {
    let (mut n, mut d) = (1, 1);
    from_fn(move || {
        if n > THRESHOLD {
            n /= 10;
            d /= 10;
        }
        d += n;
        n = (2 * d) - n;
        Some((n, d))
    })
}
