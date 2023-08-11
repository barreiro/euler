// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::cast::Cast;
use algorithm::digits::{Digits, from_raw_digits, skip_digits};
use algorithm::filter::{is_prime, less_than_u64};
use algorithm::prime::generator_wheel;
use algorithm::root::pow_10_usize;
use Solver;

/// By replacing the `1st` digit of the `2-digit` number `*3`, it turns out that six of the nine possible values: `13`, `23`, `43`, `53`, `73`, and `83`, are all prime.
/// By replacing the `3rd` and `4th` digits of `56**3` with the same digit, this `5-digit` number is the first example having seven primes among the ten generated numbers, yielding the family: `56003, 56113, 56333, 56443, 56663, 56773, and 56993`.
/// Consequently `56003`, being the first member of this family, is the smallest prime with this property.
/// Find the smallest prime which, by replacing part of the number (not necessarily adjacent digits) with the same digit, is part of an eight prime value family.
pub struct Solver051 {
    pub n: u8,
}

impl Default for Solver051 {
    fn default() -> Self {
        Self { n: 8 }
    }
}

impl Solver for Solver051 {
    fn solve(&self) -> i64 {
        let predicate = |&p: &u64| {
            // n repeated digits are required in order to produce a family of size 5 + n
            let (digit, required_repetitions) = (Digits::from(skip_digits(p, 1)), self.n - 5);

            // find a digit that is repeated the required amount of times and replace it's occurrences
            (0..=9 - self.n).find(|&i| digit.iter().skip(1).filter(|&&d| d == i).count() >= required_repetitions as usize).map_or(false, |repeated| {
                // NOTE: the digit can be repeated more times than required. to be correct should try only subsets of the repeated digit in that case.
                digit.first().expect("Digit should not be empty") != &repeated && (0..=9).filter(|&substitute| {
                    let candidate = from_raw_digits(&digit.iter().map(|&d| if d == repeated { substitute } else { d }).collect::<Vec<_>>());
                    candidate < p || !is_prime(&candidate)
                    // count the elements *not* in the family, to a allow an early return
                }).nth(1 + 9 - self.n as usize).is_none()
            })
        };

        generator_wheel().skip_while(less_than_u64(pow_10_usize(self.n as usize - 5))).find(predicate).as_i64()
    }
}
