// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::long::{factorial, incrementing_digits};
use euler::Solver;

// 145 is a curious number, as 1! + 4! + 5! = 1 + 24 + 120 = 145.
// Find the sum of all numbers which are equal to the sum of the factorial of their digits.
// Note: as 1! = 1 and 2! = 2 are not sums they are not included.

const FACTORIAL_CACHE: &[usize] = &[1, 1, 2, 6, 24, 120, 720, 5_040, 40_320, 362_880];

pub struct Solver034 {
    pub n: isize,
}

impl Default for Solver034 {
    fn default() -> Self {
        Solver034 { n: factorial(9) }
    }
}

impl Solver for Solver034 {
    fn solve(&self) -> isize {
        let equals_factorial_sum = |(n, l): &(_, Vec<_>)| *n == l.iter().map(|&d| FACTORIAL_CACHE[d as usize]).sum();
        incrementing_digits(0).enumerate().skip(3).take(self.n as _).filter(equals_factorial_sum).map(|(n, _)| n as isize).sum()
    }
}
