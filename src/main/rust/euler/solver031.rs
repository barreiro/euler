// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::combinatorics::partition;
use euler::Solver;

// In England the currency is made up of pound, £, and pence, p, and there are eight coins in general circulation: 1p, 2p, 5p, 10p, 20p, 50p, £1 (100p) and £2 (200p).
// It is possible to make £2 in the following way: 1×£1 + 1×50p + 2×20p + 1×5p + 1×2p + 3×1p
// How many different ways can £2 be made using any number of coins?

//var currency = map[int]bool{1: true, 2: true, 5: true, 10: true, 20: true, 50: true, 100: true, 200: true}

const CURRENCY: &[isize] = &[1, 2, 5, 10, 20, 50, 100, 200];

pub struct Solver031<'a> {
    pub n: isize,
    pub currency: &'a [isize],
}

impl<'a> Default for Solver031<'a> {
    fn default() -> Self {
        Solver031 {
            n: 200,
            currency: CURRENCY,
        }
    }
}

impl<'a> Solver for Solver031<'a> {
    fn solve(&self) -> isize {
        partition(self.n, self.currency)
    }
}
