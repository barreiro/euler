// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::combinatorics::choose;
use euler::Solver;

// There are exactly ten ways of selecting three from five, 12345:
//
// 123, 124, 125, 134, 135, 145, 234, 235, 245, and 345
//
// In combinatorics, we use the notation, (5 3) = 10.
// In general, (n r) = n! / r! * (n−r)!, where r ≤ n, n! = n * (n − 1) * ... * 3 * 2 * 1, and 0! = 1.
// It is not until n = 23, that a value exceeds one-million: (23 10) = 1144066.
// How many, not necessarily distinct, values of (n r) for 1 ≤ n ≤ 100, are greater than one-million?

const LIMIT: isize = 1_000_000;

pub struct Solver053 {
    pub n: isize
}

impl Default for Solver053 {
    fn default() -> Self {
        Solver053 { n: 100 }
    }
}

impl Solver for Solver053 {
    fn solve(&self) -> isize {
        // for each n, find the first combination greater than LIMIT and calculate how many there are
        (1..=self.n).map(|n| (1..=n / 2).find(|&r| choose(n, r) > LIMIT).map_or(0, |r| n + 1 - 2 * r)).sum()
    }
}
