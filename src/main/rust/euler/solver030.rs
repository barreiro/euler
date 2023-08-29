// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::cast::Cast;
use algorithm::digits::Digits;
use algorithm::root::pow;
use Solver;

/// Surprisingly there are only three numbers that can be written as the sum of fourth powers of their digits:
/// `1634 = 1^4 + 6^4 + 3^4 + 4^4`
/// `8208 = 8^4 + 2^4 + 0^4 + 8^4`
/// `9474 = 9^4 + 4^4 + 7^4 + 4^4`
/// As `1 = 1^4` is not a sum it is not included.
///
/// The sum of these numbers is `1634 + 8208 + 9474 = 19316`.
/// Find the sum of all the numbers that can be written as the sum of fifth powers of their digits.
pub struct Solver030 {
    pub n: i64,
}

impl Default for Solver030 {
    fn default() -> Self {
        Self { n: 5 }
    }
}

impl Solver for Solver030 {
    fn problem_name(&self) -> &str { "Digit fifth powers" }

    fn solve(&self) -> i64 {
        let (lower, upper) = (pow(9, self.n / 2), self.n * pow(9, self.n));
        let equals_sum_of_digit_powers = |&n : &i64| n == Digits::from(n).into_iter().map(|digit| pow(digit.as_i64(), self.n)).sum();

        (lower..upper).filter(equals_sum_of_digit_powers).sum()
    }
}
