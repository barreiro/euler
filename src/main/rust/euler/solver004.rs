// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::factor::has_factor_below;
use euler::algorithm::long::{is_palindrome, pow_10, square, decrementing};
use euler::Solver;

// A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
// Find the largest palindrome made from the product of two 3-digit numbers.

pub struct Solver004 {
    pub n: isize
}

impl Default for Solver004 {
    fn default() -> Self {
        Solver004 { n: 3 }
    }
}

impl Solver for Solver004 {
    fn solve(&self) -> isize {
        decrementing(square(pow_10(self.n))).find(|&p| is_palindrome(p) && has_factor_below(p, pow_10(self.n))).unwrap()
    }
}
