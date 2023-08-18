// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::cast::Cast;
use algorithm::factor::has_factor_pair_below;
use algorithm::filter::is_palindrome;
use algorithm::root::pow_10;
use Solver;

/// A palindromic number reads the same both ways.
/// The largest palindrome made from the product of two `2-digit` numbers is `9009 = 91 × 99`.
/// Find the largest palindrome made from the product of two `3-digit` numbers.
pub struct Solver004 {
    pub n: u64
}

impl Default for Solver004 {
    fn default() -> Self {
        Self { n: 3 }
    }
}

impl Solver for Solver004 {
    fn solve(&self) -> i64 {
        (1..=pow_10(self.n * 2)).rev().filter(is_palindrome).find(|&p| has_factor_pair_below(p.as_i64(), pow_10(self.n).as_i64())).as_i64()
    }
}
