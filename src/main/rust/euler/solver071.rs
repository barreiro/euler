// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::continued_fraction::{continued_expansion_rational, convergent_with_expansion};
use euler::Solver;

// Consider the fraction, n/d, where n and d are positive integers. If n<d and HCF(n,d)=1, it is called a reduced proper fraction.
// If we list the set of reduced proper fractions for d ≤ 8 in ascending order of size, we get:
//
// 1/8, 1/7, 1/6, 1/5, 1/4, 2/7, 1/3, 3/8, 2/5, 3/7, 1/2, 4/7, 3/5, 5/8, 2/3, 5/7, 3/4, 4/5, 5/6, 6/7, 7/8
//
// It can be seen that 2/5 is the fraction immediately to the left of 3/7.
// By listing the set of reduced proper fractions for d ≤ 1,000,000 in ascending order of size, find the numerator of the fraction immediately to the left of 3/7.

pub const BASE: (isize, isize) = (3, 7);

pub struct Solver071 {
    pub n: isize
}

impl Default for Solver071 {
    fn default() -> Self {
        Solver071 { n: 1_000_000 }
    }
}

impl Solver for Solver071 {
    fn solve(&self) -> isize {
        let expansion = continued_expansion_rational(BASE.0 * self.n - 1, BASE.1 * self.n);
        convergent_with_expansion(&expansion).take_while(|(_, d)| d[0] <= self.n).map(|(n, _)| n[0]).last().unwrap()
    }
}
