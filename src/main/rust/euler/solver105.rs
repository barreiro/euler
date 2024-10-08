// COPYRIGHT (C) 2022 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use crate::algorithm::cast::to_i64;
use crate::algorithm::io::load_default_data;
use crate::algorithm::vec::array_sum_u64;
use crate::Solver;
use crate::solver103::is_special_sum;

/// Let `S(A)` represent the sum of elements in set `A` of size `n`. We shall call it a special sum set if for any two non-empty disjoint subsets, `B` and `C`, the following properties are true:
///
/// `S(B) ≠ S(C)`; that is, sums of subsets cannot be equal.
/// If `B` contains more elements than `C` then `S(B) > S(C)`.
///
/// For example, `{81, 88, 75, 42, 87, 84, 86, 65}` is not a special sum set because `65 + 87 + 88 = 75 + 81 + 84`, whereas `{157, 150, 164, 119, 79, 159, 161, 139, 158}` satisfies both rules for all possible subset pair combinations and `S(A) = 1286`.
///
/// Using sets.txt (right click and "Save Link/Target As..."), a 4K text file with one-hundred sets containing seven to twelve elements (the two examples given above are the first two sets in the file), identify all the special sum sets, `A1, A2, ..., Ak`, and find the value of `S(A1) + S(A2) + ... + S(Ak)`.
///
/// NOTE: This problem is related to *Problem 103* and *Problem 106*.
pub struct Solver105 {
    pub n: usize,
    pub input: String,
}

impl Default for Solver105 {
    fn default() -> Self {
        Self { n: 100, input: load_default_data(105) }
    }
}

impl Solver for Solver105 {
    fn problem_name(&self) -> &str { "Special subset sums: testing" }

    fn solve(&self) -> i64 {
        let as_sorted = |line: &str| {
            let mut set = line.split(',').filter_map(|v| v.parse::<u64>().ok()).collect::<Vec<_>>();
            set.sort_unstable();
            set
        };

        self.input.lines().take(self.n).map(as_sorted).filter(|s| is_special_sum(s)).map(|s| array_sum_u64(&s)).map(to_i64).sum()
    }
}
