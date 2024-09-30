// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use crate::algorithm::cast::{Cast, char_as_i64};
use crate::algorithm::io::load_default_data;
use crate::algorithm::long::is_triangle;
use crate::Solver;

/// The `n-th` term of the sequence of triangle numbers is given by, `t(n) = 1/2 n (n+1)`; so the first ten triangle numbers are:
/// `1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...`
///
/// By converting each letter in a word to a number corresponding to its alphabetical position and adding these values we form a word value.
/// For example, the word value for `SKY` is `19 + 11 + 25 = 55 = t(10)`.
///
/// If the word value is a triangle number then we shall call the word a triangle word.
/// Using `words.txt` (right click and 'Save Link/Target As...'), a 16K text file containing nearly two-thousand common English words, how many are triangle words?
pub struct Solver042 {
    pub n: usize,
    pub input: String,
}

impl Default for Solver042 {
    fn default() -> Self {
        Self { n: 1786, input: load_default_data(42) }
    }
}

impl Solver for Solver042 {
    fn problem_name(&self) -> &str { "Coded triangle numbers" }

    fn solve(&self) -> i64 {
        let char_sum = |name: &str| name.chars().map(char_as_i64).sum();
        self.input.split(',').map(|s| s.trim_matches('\"')).take(self.n).map(char_sum).filter(is_triangle).count().as_i64()
    }
}
