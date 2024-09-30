// COPYRIGHT (C) 2022 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use crate::algorithm::cast::to_i64;
use crate::algorithm::combinatorics::choose;
use crate::Solver;

/// Let `S(A)` represent the sum of elements in set `A` of size `n`. We shall call it a special sum set if for any two non-empty disjoint subsets, `B` and `C`, the following properties are true:
///
/// `S(B) ≠ S(C)`; that is, sums of subsets cannot be equal.
/// If `B` contains more elements than `C` then `S(B) > S(C)`.
/// For this problem we shall assume that a given set contains `n` strictly increasing elements, and it already satisfies the second rule.
///
/// Surprisingly, out of the `25` possible subset pairs that can be obtained from a set for which `n = 4`, only `1` of these pairs need to be tested for equality (first rule).
/// Similarly, when `n = 7`, only `70` out of the `966` subset pairs need to be tested.
///
/// For `n = 12`, how many of the `261625` subset pairs that can be obtained need to be tested for equality?
///
/// NOTE: This problem is related to *Problem 103* and *Problem 105*.
pub struct Solver106 {
    pub n: u64,
}

impl Default for Solver106 {
    fn default() -> Self {
        Self { n: 12 }
    }
}

impl Solver for Solver106 {
    fn problem_name(&self) -> &str { "Special subset sums: meta-testing" }

    fn solve(&self) -> i64 {
        // given a size `2 < i ≤ n/2`, there are `choose(n, 2*i)` ways to form two subsets of size `i`
        // given a subset of size `2*i (4, 6, 8, 10, ...)`, the number of subsets that need to be tested is given by the sequence `(1, 5, 21, 84, …)`
        // these are the number of combinations where there is not a clear inequality relationship, in other words, it's not possible to establish that for each element in B there is a corresponding element in C that is greater.
        (2..=self.n / 2).map(|i| choose(self.n, 2 * i) * choose(2 * i - 1, i - 2)).map(to_i64).sum()
    }
}
