// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::combinatorics::pythagorean_triplets;
use euler::algorithm::long::cube;
use euler::Solver;

// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which, a^2 + b^2 = c^2
// For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.
//
// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
// Find the product abc.

pub struct Solver009 {
    pub n: isize,
}

impl Default for Solver009 {
    fn default() -> Self {
        Solver009 { n: 1000 }
    }
}

impl Solver for Solver009 {
    fn solve(&self) -> isize {
        // can be the product of a primitive triplet or a multiple
        let product = |(a, b, c)| a * b * c * cube(self.n / (a + b + c));
        pythagorean_triplets().take_while(|&(a, _, _)| a << 1 < self.n).find(|&(a, b, c)| self.n % (a + b + c) == 0).map(product).unwrap_or_default()
    }
}
