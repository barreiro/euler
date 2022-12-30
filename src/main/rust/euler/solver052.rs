// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::convert::TryFrom;

use algorithm::bit::BitSet;
use algorithm::cast::Cast;
use algorithm::digits::{Digits, digits_sum, from_raw_digits};
use Solver;

/// It can be seen that the number, `125874`, and its double, `251748`, contain exactly the same digits, but in a different order.
/// Find the smallest positive integer, `x`, such that `2x, 3x, 4x, 5x, and 6x` contain the same digits.
pub struct Solver052 {
    pub n: u64,
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
        (from_raw_digits(&(1..=u8::try_from(self.n).unwrap()).rev().collect::<Vec<_>>())..).filter(|&candidate| {
            let candidate_sum = digits_sum(candidate);
            (2..=self.n).map(|m| m * candidate).all(|multiple| digits_sum(multiple) == candidate_sum)
        }).find(|&candidate| {
            let set = Digits::from(candidate).into_iter().collect::<BitSet>();
            (2..=self.n).map(|m| m * candidate).all(|multiple| Digits::from(multiple).into_iter().all(|m| set.contains(m)))
        }).as_i64()
    }
}
