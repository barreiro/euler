// COPYRIGHT (C) 2022 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::cast::Cast;
use algorithm::combinatorics::permutations_of_set_with;
use algorithm::digits::concatenation;
use algorithm::vec::{all_unique_by, array_sum_u64};
use Solver;

/// Let `S(A)` represent the sum of elements in set `A` of size `n`. We shall call it a special sum set if for any two non-empty disjoint subsets, `B` and `C`, the following properties are true:
///
/// `S(B) ≠ S(C)`; that is, sums of subsets cannot be equal.
/// If `B` contains more elements than `C` then `S(B) > S(C)`.
///
/// If `S(A)` is minimised for a given `n`, we shall call it an optimum special sum set. The first five optimum special sum sets are given below.
/// ```
/// n = 1: {1}
/// n = 2: {1, 2}
/// n = 3: {2, 3, 4}
/// n = 4: {3, 5, 6, 7}
/// n = 5: {6, 9, 11, 12, 13}
/// ```
/// It seems that for a given optimum set, `A = {a1, a2, ... , an}`, the next optimum set is of the form `B = {b, a1+b, a2+b, ... ,an+b}`, where b is the "middle" element on the previous row.
/// By applying this "rule" we would expect the optimum set for `n = 6` to be `A = {11, 17, 20, 22, 23, 24}`, with `S(A) = 117`.
/// However, this is not the optimum set, as we have merely applied an algorithm to provide a near optimum set.
/// The optimum set for `n = 6` is `A = {11, 18, 19, 20, 22, 25}`, with `S(A) = 115` and corresponding set string: `111819202225`.
///
/// Given that `A` is an optimum special sum set for `n = 7`, find its set string.
///
/// NOTE: This problem is related to *Problem 105* and *Problem 106*.
pub struct Solver103 {
    pub n: usize,
}

impl Default for Solver103 {
    fn default() -> Self {
        Self { n: 7 }
    }
}

impl Solver for Solver103 {
    fn problem_name(&self) -> &str { "Special subset sums: optimum" }

    fn solve(&self) -> i64 {
        set_from_enhanced_formula(self.n).iter().fold(0, |a, &b| concatenation(a, b)).as_i64()
    }
}

// --- //

// recursive function that uses the formula suggested in the problem to discover sets of increasing size
fn set_from_enhanced_formula(size: usize) -> Vec<u64> {
    if size == 1 {
        vec![1]
    } else {
        let previous = set_from_enhanced_formula(size - 1);
        let mut set = vec![previous[previous.len() / 2]];
        let mut diff = previous.iter().scan(0, |state, &element|{
            let d = element - *state;
            *state = element;
            Some(d)
        }).collect::<Vec<_>>();

        // the enhancement happens when there is a difference of 6 in the elements of the set
        // in that case, use 7 instead and find a permutation of the differences that yields a special sum set
        if diff.contains(&6) {
            diff.iter_mut().filter(|d| **d == 6).for_each(|d| *d = 7);
            diff.reverse(); // permutations_of_set requires descending order
            permutations_of_set_with(diff, |d| {
                let mut new_set = set.clone();
                new_set.append(&mut d.iter().scan(set[0], |state, &diff| {
                    *state += diff;
                    Some(*state)
                }).collect());
                is_special_sum(&new_set).then_some(new_set)
            }).next().expect("Permutation should exist") // because the way permutations_of_set works, the first permutation found has the least sum
        } else {
            previous.iter().for_each(|p| set.push(set[0] + p));
            set
        }
    }
}

/// checks that a given set (in ascending order) has all subsets with different sums and any smaller subset has a smaller sum
#[must_use]
pub fn is_special_sum(set: &Vec<u64>) -> bool {
    // checks condition `B > C => S(B) > S(C)` by checking that the sum of the first half is bigger than the second half
    let by_size = || array_sum_u64(&set[0..(1 + set.len()) / 2]) > array_sum_u64(&set[1 + set.len() / 2..]);

    // checks that a sum does not exists in subsets
    let by_subset = || {
        let sums = set.iter().fold(Vec::with_capacity(1 << set.len()), |mut sums, &s| {
            let current_capacity = sums.len();
            (0..current_capacity).for_each(|i| sums.push(sums[i] + s));
            sums.push(s);
            sums
        });
        all_unique_by(&sums, |&s| s)
    };

    by_size() && by_subset()
}
