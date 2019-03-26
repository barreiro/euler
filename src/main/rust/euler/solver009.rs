// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::Solver;
use euler::algorithm::long::{int_sqrt,square};

// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which, a^2 + b^2 = c^2
// For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.
//
// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
// Find the product abc.

pub struct Solver009 {
    pub n: isize
}

impl Default for Solver009 {
    fn default() -> Self {
        Solver009 { n: 1000 }
    }
}

impl Solver for Solver009 {
    fn solve(&self) -> isize {
        // solved with Euclides Formula --- a=m^2-n^2 --- b=2nm --- c=m^2+n^2 --- with m>n

        for m in 2..int_sqrt(self.n) {
            for n in 1..m {
                let (a, b, c) = (square(m) - square(n), 2 * m * n, square(m) + square(n));
                if a + b + c == self.n {
                    return a * b * c;
                }
            }
        }
        0
    }
}
