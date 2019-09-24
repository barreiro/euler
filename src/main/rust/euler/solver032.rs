// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::bit::bit_set;
use euler::algorithm::combinatorics::permutations_with;
use euler::algorithm::long::from_digits_index;
use euler::Solver;

// We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n exactly once; for example, the 5-digit number, 15234, is 1 through 5 pandigital.
// The product 7254 is unusual, as the identity, 39 × 186 = 7254, containing multiplicand, multiplier, and product is 1 through 9 pandigital.
// Find the sum of all products whose multiplicand/multiplier/product identity can be written as a 1 through 9 pandigital.
// HINT: Some products can be obtained in more than one way so be sure to only include it once in your sum.

pub struct Solver032 {
    pub n: isize,
}

impl Default for Solver032 {
    fn default() -> Self {
        Solver032 { n: 9 }
    }
}

impl Solver for Solver032 {
    fn solve(&self) -> isize {
        let mut set = bit_set();

        permutations_with(1, self.n, |p| {
            for i in 1..p.len() - 1 {
                for j in i + 1..p.len() {
                    let (a, b, c) = (from_digits_index(p, 0, i), from_digits_index(p, i, j), from_digits_index(p, j, p.len()));
                    if a == b * c {
                        return Some(a);
                    }
                }
            }
            None
        }).filter(|&a| set.insert(a)).sum()
    }
}
