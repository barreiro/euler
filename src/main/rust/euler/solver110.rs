// COPYRIGHT (C) 2022 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use Solver;
use Solver108;

/// In the following equation `x`, `y`, and `n` are positive integers.
/// ```
/// 1 / x + 1 / y = 1 / n
/// ```
/// It can be verified that when `1260` there are `113` distinct solutions and this is the least value of `n` for which the total number of distinct solutions exceeds one hundred.
/// What is the least value of `n` for which the number of distinct solutions exceeds four million?
///
/// NOTE: This problem is a much more difficult version of Problem 108 and as it is well beyond the limitations of a brute force approach it requires a clever implementation.
pub struct Solver110 {
    pub n: u64,
}

impl Default for Solver110 {
    fn default() -> Self {
        Self { n: 4_000_000 }
    }
}

impl Solver for Solver110 {
    fn solve(&self) -> i64 {
        Solver108 { n: self.n }.solve() // delegate to problem 108
    }
}
