// COPYRIGHT (C) 2023 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::cast::Cast;
use Solver;
use solver114::multipart;

const THRESHOLD: u64 = 1_000_000;

/// NOTE: This is a more difficult version of *Problem 114*.
///
/// A row measuring `n` units in length has red blocks with a minimum length of `m` units placed on it, such that any two red blocks (which are allowed to be different lengths) are separated by at least one black square.
///
/// Let the fill-count function, `F(m,n)`, represent the number of ways that a row can be filled.
/// For example, `F(3,29) = 673135` and `F(3,30) = 1089155`.
///
/// That is, for `m = 3`, it can be seen that `n = 30` is the smallest value for which the fill-count function first exceeds one million.
///
/// In the same way, for `m = 10`, it can be verified that `F(10,56) = 880711` and `F(10,57) = 1184904`, so `m = 57` is the least value for which the fill-count function first exceeds one million.
///
/// For `n = 50`, find the least value of for which the fill-count function first exceeds one million.
pub struct Solver115 {
    pub n: u64,
    pub threshold: u64,
}

impl Default for Solver115 {
    fn default() -> Self {
        Self { n: 50, threshold: THRESHOLD }
    }
}

impl Solver for Solver115 {
    fn problem_name(&self) -> &str { "Counting block combinations II" }

    #[allow(clippy::maybe_infinite_iter)]
    fn solve(&self) -> i64 {
        (self.n..).find(|&set_size| multipart(set_size, self.n) > self.threshold).as_i64()
    }
}
