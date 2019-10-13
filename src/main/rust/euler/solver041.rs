// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::combinatorics::permutations_with;
use euler::algorithm::long::{from_digits_index, is_even};
use euler::algorithm::prime::miller_rabin;
use euler::Solver;

// We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n exactly once.
// For example, 2143 is a 4-digit pandigital and is also prime.
// What is the largest n-digit pandigital prime that exists?

pub struct Solver041 {
    pub n: isize,
}

impl Default for Solver041 {
    fn default() -> Self {
        Solver041 { n: 9 }
    }
}

impl Solver for Solver041 {
    fn solve(&self) -> isize {
        // Assume the largest prime also start with the biggest digit
        let predicate = |d: &[_]| if *d.last().unwrap() != d.len() as _ || is_even(*d.first().unwrap()) { None } else {
            let candidate = from_digits_index(d, 0, d.len());
            if miller_rabin(candidate) { Some(candidate) } else { None }
        };

        // If the sum of the digits of the permutation is multiple of three, all permutations are multiple of three as well
        (1..=self.n).rev().filter(|&n| n % 3 != 0).find_map(|n| permutations_with(1, n, predicate).max()).unwrap()
    }
}
