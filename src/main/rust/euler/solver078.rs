// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::combinatorics::partition_modulo_find;
use euler::Solver;

// Let p(n) represent the number of different ways in which n coins can be separated into piles.
// For example, five coins can be separated into piles in exactly seven different ways, so p(5)=7.
//
// OOOOO
// OOOO   O
// OOO   OO
// OOO   O   O
// OO   OO   O
// OO   O   O   O
// O   O   O   O   O
//
// Find the least value of n for which p(n) is divisible by one million.

pub struct Solver078 {
    pub n: isize
}

impl Default for Solver078 {
    fn default() -> Self {
        Solver078 { n: 1_000_000 }
    }
}

impl Solver for Solver078 {
    fn solve(&self) -> isize {
        partition_modulo_find(self.n, 0)
    }
}
