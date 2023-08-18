// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::bit::BitSet;
use algorithm::cast::Cast;
use algorithm::digits::{Digit, Digits, digits_sum, from_raw_digits};
use Solver;

/// It can be seen that the number, `125874`, and its double, `251748`, contain exactly the same digits, but in a different order.
/// Find the smallest positive integer, `x`, such that `2x, 3x, 4x, 5x, and 6x` contain the same digits.
pub struct Solver052 {
    pub n: Digit,
}

impl Default for Solver052 {
    fn default() -> Self {
        Self { n: 6 }
    }
}

#[allow(clippy::maybe_infinite_iter)]
impl Solver for Solver052 {
    fn solve(&self) -> i64 {
        // start on the number 123...n and do a preliminary filter based on the sum of the digits
        (from_raw_digits(&(1..=self.n).rev().collect::<Vec<_>>())..).filter(|&candidate| {
            (2..=u64::from(self.n)).map(|m| m * candidate).all(|multiple| digits_sum(multiple) == digits_sum(candidate))
        }).find(|&candidate| {
            let set = Digits::from(candidate).into_iter().collect::<BitSet>();
            (2..=u64::from(self.n)).map(|m| m * candidate).all(|multiple| Digits::from(multiple).into_iter().all(|m| set.contains(m)))
        }).as_i64()
    }
}
