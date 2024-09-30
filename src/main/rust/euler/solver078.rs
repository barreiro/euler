// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use crate::algorithm::cast::Cast;
use crate::algorithm::combinatorics::partition_modulo_find;
use crate::Solver;

/// Let `p(n)` represent the number of different ways in which `n` coins can be separated into piles.
/// For example, five coins can be separated into piles in exactly seven different ways, so `p(5)=7`.
/// ```
/// OOOOO
/// OOOO  O
/// OOO  OO
/// OOO  O  O
/// OO  OO  O
/// OO  O  O  O
/// O  O  O  O  O
/// ```
/// Find the least value of `n` for which `p(n)` is divisible by one million.
pub struct Solver078 {
    pub n: u64
}

impl Default for Solver078 {
    fn default() -> Self {
        Self { n: 1_000_000 }
    }
}

impl Solver for Solver078 {
    fn problem_name(&self) -> &str { "Coin partitions" }

    fn solve(&self) -> i64 {
        partition_modulo_find(self.n, 0).as_i64()
    }
}
