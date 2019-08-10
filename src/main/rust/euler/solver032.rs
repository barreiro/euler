// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::combinatorics::permutation_array;
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
        Solver032 {
            n: 9
        }
    }
}

impl Solver for Solver032 {
    fn solve(&self) -> isize {
        // products are found together, so for deduplication just compare with the previous seen
        let sum_deduplication = |state: &mut isize, a| {
            let dup = *state == a;
            *state = a;
            if !dup { Some(a) } else { Some(0) }
        };

        permutation_array((1..=self.n).collect::<Vec<_>>()).filter_map(|p| {
            for i in 1..self.n as usize - 1 {
                for j in i + 1..self.n as usize {
                    let (a, b, c) = (from_digits_index(&p, 0, i), from_digits_index(&p, i, j), from_digits_index(&p, j, self.n as usize));
                    if a == b * c {
                        return Some(a);
                    }
                }
            }
            return None;
        }).scan(0, sum_deduplication).sum()
    }
}