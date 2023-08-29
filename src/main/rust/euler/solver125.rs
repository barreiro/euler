// COPYRIGHT (C) 2023 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::collections::HashSet;

use algorithm::cast::to_i64;
use algorithm::filter::{is_palindrome, less_than_u64};
use algorithm::root::{pow_10, square_u64};
use Solver;

/// The palindromic number `595` is interesting because it can be written as the sum of consecutive squares: `6^2 + 7^2 + 8^2 + 9^2 + 10^2 + 11^2 + 12^2`.
///
/// There are exactly eleven palindromes below one-thousand that can be written as consecutive square sums, and the sum of these palindromes is `4164`.
/// Note that `1 = 0^2 + 1^2` has not been included as this problem is concerned with the squares of positive integers.
///
/// Find the sum of all the numbers less than `10^8` that are both palindromic and can be written as the sum of consecutive squares.
pub struct Solver125 {
    pub n: u64,
}

impl Default for Solver125 {
    fn default() -> Self {
        Self { n: 8 }
    }
}

#[allow(clippy::maybe_infinite_iter)]
impl Solver for Solver125 {
    fn solve(&self) -> i64 {
        // let squares = (1..).map(square_u64).take_while(less_than_u64(pow_10(self.n) / 2)).collect::<Vec<_>>();
        // palindromes().take_while(less_than_u64(pow_10(self.n))).filter(|&p| {
        //     (2..=squares.len()).any(|w| squares.windows(w).map(array_sum_u64).take_while(less_or_equal_than_u64(p)).any(|sum| sum == p))
        // }).map(to_i64).sum()

        // let _palindromes = palindromes().take_while(less_than_u64(pow_10(self.n))).collect::<Vec<_>>();
        // (2..=squares.len())
        //     .flat_map(|w| squares.windows(w).map(array_sum_u64).take_while(less_than_u64(pow_10(self.n))))
        //     .filter(|candidate| _palindromes.binary_search(candidate).is_ok())
        //     .collect::<HashSet<_>>()
        //     .into_iter()
        //     .map(to_i64)
        //     .sum()

        let squares = (1..).map(square_u64).take_while(less_than_u64(pow_10(self.n) / 2)).collect::<Vec<_>>();
        (0..squares.len()).flat_map(|i| squares[i + 1..].iter().scan(squares[i], |state, &j| {
            *state += j;
            Some(*state).filter(less_than_u64(pow_10(self.n)))
        })).filter(is_palindrome).collect::<HashSet<_>>().into_iter().map(to_i64).sum()
    }
}
