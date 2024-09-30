// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use crate::algorithm::cast::{Cast, to_i64};
use crate::algorithm::combinatorics::permutations_of_digits_with;
use crate::algorithm::digits::{Digit, from_raw_digits};
use crate::algorithm::prime::generator_trial_division;
use crate::Solver;

const DIM: usize = 3;

/// The number, `1406357289`, is a `0` to `9` pandigital number because it is made up of each of the digits `0` to `9` in some order, but it also has a rather interesting sub-string divisibility property.
/// Let `d(1)` be the `1st` digit, `d(2)` be the `2nd` digit, and so on. In this way, we note the following:
/// ```
/// d(2) d(3) d(4) = 406 is divisible by 2
/// d(3) d(4) d(5) = 063 is divisible by 3
/// d(4) d(5) d(6) = 635 is divisible by 5
/// d(5) d(6) d(7) = 357 is divisible by 7
/// d(6) d(7) d(8) = 572 is divisible by 11
/// d(7) d(8) d(9) = 728 is divisible by 13
/// d(8) d(9) d(10) = 289 is divisible by 17
/// ```
/// Find the sum of all `0` to `9` pandigital numbers with this property.
pub struct Solver043 {
    pub n: Digit,
}

impl Default for Solver043 {
    fn default() -> Self {
        Self { n: 9 }
    }
}

impl Solver for Solver043 {
    fn problem_name(&self) -> &str { "Sub-string divisibility" }

    fn solve(&self) -> i64 {
        let primes = generator_trial_division().take(self.n as usize - DIM + 1).collect::<Vec<_>>();
        let predicate = |d: &[Digit]| (0..primes.len()).find(|&n| from_raw_digits(&d[n..n + DIM]) % primes[primes.len() - n - 1] != 0).map(|n| n.as_u64()).xor(Some(from_raw_digits(d)));
        permutations_of_digits_with(0, self.n, predicate).map(to_i64).sum()
    }
}
