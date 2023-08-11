// COPYRIGHT (C) 2022 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::iter::from_fn;

use algorithm::cast::to_i64;
use algorithm::combinatorics::choose;
use Solver;

/// Let `S(A)` represent the sum of elements in set `A` of size `n`. We shall call it a special sum set if for any two non-empty disjoint subsets, `B` and `C`, the following properties are true:
///
/// `S(B) ≠ S(C)`; that is, sums of subsets cannot be equal.
/// If `B` contains more elements than `C` then `S(B) > S(C)`.
/// For this problem we shall assume that a given set contains `n` strictly increasing elements and it already satisfies the second rule.
///
/// Surprisingly, out of the `25` possible subset pairs that can be obtained from a set for which `n = 4`, only `1` of these pairs need to be tested for equality (first rule).
/// Similarly, when `n = 7`, only `70` out of the `966` subset pairs need to be tested.
///
/// For `n = 12`, how many of the `261625` subset pairs that can be obtained need to be tested for equality?
///
/// NOTE: This problem is related to Problem 103 and Problem 105.
pub struct Solver106 {
    pub n: u64,
}

impl Default for Solver106 {
    fn default() -> Self {
        Self { n: 12 }
    }
}

impl Solver for Solver106 {
    fn solve(&self) -> i64 {
        // There are (3^n - 2^(n+1) + 1) / 2 pairs of non-empty subsets.
        // We either put an element of A in three sets: B, C or NONE. There are 3^n possibilities to do that, but they include the cases where B or C are empty (or both), the number of these cases has to be subtracted from the result.
        // B and C can be swapped, so the total number is twice the number of effective subsets.

        // count the number of subset partitions where elements of one set, element by element, are not always bigger/smaller than elements of another set
        // (2..=self.n >> 1).map(|subset_size| {
        //     // use Gosper's Hack to generate all the subsets with a given number of elements. subsets are represented as the bits on an integer
        //     let subsets = gospers(self.n, subset_size).collect::<Vec<_>>();
        //     (1..subsets.len()).map(|i| (0..i).filter(|&j| subsets[i] & subsets[j] == 0).filter(|&j| {
        //         // subsets[i] < subsets[j] ... compare the least significant digits of the two integers then unset them while the inequality holds
        //         let (mut b, mut c) = (subsets[i], subsets[j]);
        //         while b != 0 && b & !(b - 1) > c & !(c - 1) {
        //             b &= b - 1;
        //             c &= c - 1;
        //         }
        //         b != 0
        //     }).count().as_i64()).sum::<i64>()
        // }).sum()

        // given a size 2 < i <= n/2, there are choose(n, 2*i) ways to form two subsets of size i
        // given a subset of size 2*i (4, 6, 8, 10, ...), the number of subsets that need to be tested is given by the sequence (1, 5, 21, 84, ...)
        // these are the number of combinations where there is not a clear inequality relationship, in other words, it's not possible to establish that for each element in B there is a corresponding element in C that is greater.
        (2..=self.n >> 1).map(|i| choose(self.n, 2 * i) * choose(2 * i - 1, i - 2)).map(to_i64).sum()
    }
}

fn _gospers(length: u64, elements: u64) -> impl Iterator<Item=u64> {
    let mut set = (1 << elements) - 1;
    from_fn(move || {
        let previous = set;
        let smallest = set & (!set + 1);
        let ripple = set + smallest;
        set = (((ripple ^ set) >> 2) / smallest) | ripple;
        Some(previous).filter(|&p| p < (1 << length))
    })
}
