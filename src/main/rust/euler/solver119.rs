// COPYRIGHT (C) 2023 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::iter::from_fn;

use algorithm::cast::Cast;
use algorithm::digits::digits_sum;
use Solver;

// enough for all `a_n` within 64 bit
const BASES: u64 = 100;

/// The number `512` is interesting because it is equal to the sum of its digits raised to some power: `5 + 1 + 2 = 8`, and `8^3 = 512`.
/// Another example of a number with this property is `614656 = 28^4`.
///
/// We shall define `a_n` to be the `n`th term of this sequence and insist that a number must contain at least two digits to have a sum.
///
/// You are given that `a_2 = 512` and `a_10 = 614656`.
///
/// Find `a_30`.
pub struct Solver119 {
    pub n: usize,
}

impl Default for Solver119 {
    fn default() -> Self {
        Self { n: 30 }
    }
}

impl Solver for Solver119 {
    fn problem_name(&self) -> &str { "Digit power sum" }

    fn solve(&self) -> i64 {
        // skip powers that contain a single digit
        powers().filter_map(|(base, power)| (digits_sum(power) == base).then_some(power)).nth(7 + self.n).as_i64()
    }
}

// --- //

// iterate over increasingly values that are powers in some base
fn powers() -> impl Iterator<Item=(i64, u64)> {
    let mut powers = (0..BASES).collect::<Vec<_>>();
    from_fn(move || {
        let min = (2..powers.len()).min_by_key(|&i| powers[i]).expect("There should be powers");
        let power = powers[min];
        powers[min] *= min.as_u64();
        Some((min.as_i64(), power))
    })
}
