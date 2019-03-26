// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::long::factorial;
use euler::algorithm::long::pow_10;
use euler::Solver;

//A permutation is an ordered arrangement of objects. For example, 3124 is one possible permutation of the digits 1, 2, 3 and 4.
//If all of the permutations are listed numerically or alphabetically, we call it lexicographic order.
//The lexicographic permutations of 0, 1 and 2 are:   012   021   102   120   201   210
//What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?

const BASE: &[isize] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

pub struct Solver024<'a> {
    pub n: isize,
    pub base: &'a [isize],
}

impl<'a> Default for Solver024<'a> {
    fn default() -> Self {
        Solver024 {
            n: 1000000,
            base: BASE,
        }
    }
}

impl<'a> Solver for Solver024<'a> {
    fn solve(&self) -> isize {
        let (mut value, mut unplaced, mut sum) = (self.n as usize - 1, Vec::from(self.base), 0);

        for l in (1..unplaced.len() as isize).rev() {
            let f = factorial(l) as usize;
            sum += pow_10(l) * unplaced[value / f];
            unplaced.remove(value / f);
            value %= f;
        }
        sum + unplaced[0]
    }
}
