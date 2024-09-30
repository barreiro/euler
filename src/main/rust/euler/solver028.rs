// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use crate::algorithm::root::square;
use crate::Solver;

/// Starting with the number `1` and moving to the right in a clockwise direction a `5` by `5` spiral is formed as follows:
/// ```
/// 21 22 23 24 25
/// 20  7  8  9 10
/// 19  6  1  2 11
/// 18  5  4  3 12
/// 17 16 15 14 13
/// ```
/// It can be verified that the sum of the numbers on the diagonals is `101`.
///
/// What is the sum of the numbers on the diagonals in a `1001` by `1001` spiral formed in the same way?
pub struct Solver028 {
    pub n: i64,
}

impl Default for Solver028 {
    fn default() -> Self {
        Self { n: 1001 }
    }
}

impl Solver for Solver028 {
    fn problem_name(&self) -> &str { "Number spiral diagonals" }

    fn solve(&self) -> i64 {
        // sum of the left corners == right corners == 2*i*i - 3*(i-1)
        1 + (3..=self.n).step_by(2).map(|i| 4 * square(i) - 6 * (i - 1)).sum::<i64>()
    }
}
