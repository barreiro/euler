// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::long::lcm;
use euler::Solver;

// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

pub struct Solver005 {
    pub n: isize
}

impl Default for Solver005 {
    fn default() -> Self {
        Solver005 { n: 20 }
    }
}

impl Solver for Solver005 {
    fn solve(&self) -> isize {
        (2..=self.n).fold(1, |acc, l| lcm(acc, l))
    }
}
