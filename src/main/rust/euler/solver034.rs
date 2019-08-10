// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::long::{factorial, incrementing_digits};
use euler::Solver;

// 145 is a curious number, as 1! + 4! + 5! = 1 + 24 + 120 = 145.
// Find the sum of all numbers which are equal to the sum of the factorial of their digits.
// Note: as 1! = 1 and 2! = 2 are not sums they are not included.

const FACTORIAL_CACHE: &[isize] = &[1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];

pub struct Solver034 {
    pub n: isize,
}

impl Default for Solver034 {
    fn default() -> Self {
        Solver034 {
            n: factorial(9)
        }
    }
}

impl Solver for Solver034 {
    fn solve(&self) -> isize {
        let fast_factorial_sum = |l: &[isize]| l.iter().map(|&d| FACTORIAL_CACHE[d as usize]).sum();

        incrementing_digits(3).take(self.n as usize).enumerate().filter_map(|(n, digits)| if 4 + n as isize == fast_factorial_sum(&digits) { Some(4 + n as isize) } else { None }).sum()
    }
}
