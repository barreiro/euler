// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::long::{is_pentagonal, pentagonal};
use euler::Solver;

// Pentagonal numbers are generated by the formula, P(n) = n * (3 * n − 1) / 2. The first ten pentagonal numbers are:
// 1, 5, 12, 22, 35, 51, 70, 92, 117, 145, ...
// It can be seen that P(4) + P(7) = 22 + 70 = 92 = P(8). However, their difference, 70 − 22 = 48, is not pentagonal.
// Find the pair of pentagonal numbers, P(j) and P(k), for which their sum and difference are pentagonal and D = |P(k) − P(j)| is minimised; what is the value of D?

pub struct Solver044 {
    pub n: isize
}

impl Default for Solver044 {
    fn default() -> Self {
        Solver044 { n: 0 }
    }
}

impl Solver for Solver044 {
    fn solve(&self) -> isize {
        let predicate = |j, k| {
            let (p_j, p_k) = (pentagonal(j), pentagonal(k));
            let p_diff = p_j - p_k;
            if is_pentagonal(p_diff) && is_pentagonal(p_j + p_k) { Some(p_diff) } else {None}
        };

        (2..).filter_map(|j| (1..j).find_map(|k| predicate(j, k))).nth(self.n as _).unwrap()
    }
}