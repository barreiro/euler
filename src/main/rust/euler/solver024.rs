// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::long::factorial;
use euler::algorithm::long::pow_10;
use euler::Solver;

// A permutation is an ordered arrangement of objects. For example, 3124 is one possible permutation of the digits 1, 2, 3 and 4.
// If all of the permutations are listed numerically or alphabetically, we call it lexicographic order.

// The lexicographic permutations of 0, 1 and 2 are: 012, 021, 102, 120, 201, 210
// What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?

const BASE: &[isize] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

pub struct Solver024 {
    pub n: isize,
    pub base: Vec<isize>,
}

impl Default for Solver024 {
    fn default() -> Self {
        Solver024 { n: 1_000_000, base: BASE.to_vec() }
    }
}

impl Solver for Solver024 {
    fn solve(&self) -> isize {
        let (mut value, mut unplaced, mut sum) = (self.n as usize - 1, self.base.to_vec(), 0);

        // use a kind of factorization of N over the factorials and in the end convert the digits to a number
        for l in (1..unplaced.len() as _).rev() {
            let f = factorial(l) as usize;
            sum += pow_10(l) * unplaced[value / f];
            unplaced.remove(value / f);
            value %= f;
        }
        sum + unplaced[0]
    }
}
