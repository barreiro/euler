// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::cast::Cast;
use algorithm::digits::{Digits, incrementing_digits};
use algorithm::long::factorial;
use Solver;

const FACTORIAL_CACHE: &[usize] = &[1, 1, 2, 6, 24, 120, 720, 5_040, 40_320, 362_880];

/// 145 is a curious number, as `1! + 4! + 5! = 1 + 24 + 120 = 145`.
/// Find the sum of all numbers which are equal to the sum of the factorial of their digits.
/// Note: as `1! = 1` and `2! = 2` are not sums they are not included.
pub struct Solver034 {
    pub n: usize,
}

impl Default for Solver034 {
    fn default() -> Self {
        Self { n: factorial(9).as_usize() }
    }
}

impl Solver for Solver034 {
    fn solve(&self) -> i64 {
        let equals_factorial_sum = |(n, digits): &(_, Digits)| *n == digits.iter().map(|&d| FACTORIAL_CACHE[usize::from(d)]).sum();
        incrementing_digits().enumerate().skip(3).take(self.n).filter(equals_factorial_sum).map(|(n, _)| n.as_i64()).sum()
    }
}
