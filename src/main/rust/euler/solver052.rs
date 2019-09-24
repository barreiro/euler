// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::bit::BitSet;
use euler::algorithm::long::{digits_sum, from_digits, to_digits};
use euler::Solver;

// It can be seen that the number, 125874, and its double, 251748, contain exactly the same digits, but in a different order.
// Find the smallest positive integer, x, such that 2x, 3x, 4x, 5x, and 6x, contain the same digits.

pub struct Solver052 {
    pub n: isize
}

impl Default for Solver052 {
    fn default() -> Self {
        Solver052 { n: 6 }
    }
}

impl Solver for Solver052 {
    fn solve(&self) -> isize {
        // start on the number 123...n and do a preliminary filter based on the sum of the digits
        (from_digits((1..=self.n).rev().collect())..).filter(|&candidate| {
            let candidate_sum = digits_sum(candidate);
            (2..=self.n).map(|m| m * candidate).all(|multiple| digits_sum(multiple) == candidate_sum)
        }).find(|&candidate| {
            let set = to_digits(candidate).iter().collect::<BitSet>();
            (2..=self.n).map(|m| m * candidate).all(|multiple| to_digits(multiple).iter().all(|&m| set.contains(m)))
        }).unwrap()
    }
}
