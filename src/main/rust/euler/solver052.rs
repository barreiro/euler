// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use crate::algorithm::bit::BitSet;
use crate::algorithm::cast::Cast;
use crate::algorithm::digits::{digits_iter, digits_sum, from_raw_digits, Digit};
use crate::Solver;

/// It can be seen that the number, `125874`, and its double, `251748`, contain exactly the same digits, but in a different order.
///
/// Find the smallest positive integer, `x`, such that `2x, 3x, 4x, 5x, and 6x` contain the same digits.
pub struct Solver052 {
    pub n: Digit,
}

impl Default for Solver052 {
    fn default() -> Self {
        Self { n: 6 }
    }
}

impl Solver for Solver052 {
    fn problem_name(&self) -> &str { "Permuted multiples" }

    #[allow(clippy::maybe_infinite_iter)]
    fn solve(&self) -> i64 {
        // start on the number 123…n and do a preliminary filter based on the sum of the digits
        (from_raw_digits(&(1..=self.n).rev().collect::<Vec<_>>())..).find(|&candidate| {
            (2..=self.n.as_u64()).map(|m| m * candidate).all(|multiple| digits_sum(multiple) == digits_sum(candidate) && {
                let digit_set = digits_iter(candidate).collect::<BitSet>();
                digits_iter(multiple).all(|d| digit_set.contains(d))
            })
        }).as_i64()
    }
}
