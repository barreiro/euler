// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::cast::to_i64;
use algorithm::digits::{Digits, palindromes};
use algorithm::filter::less_than_u64;
use Solver;

/// The decimal number, `585 = 1001001001 (binary)`, is palindromic in both bases.
///
/// Find the sum of all numbers, less than one million, which are palindromic in base `10` and base `2`.
///
/// (Please note that the palindromic number, in either base, may not include leading zeros.)
pub struct Solver036 {
    pub n: u64,
}

impl Default for Solver036 {
    fn default() -> Self {
        Self { n: 1_000_000 }
    }
}

impl Solver for Solver036 {
    fn problem_name(&self) -> &str { "Double-base palindromes" }

    fn solve(&self) -> i64 {
        palindromes().take_while(less_than_u64(self.n)).filter(|&p| Digits::from((p, 2)).is_palindrome()).map(to_i64).sum()
    }
}
