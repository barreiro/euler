// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::long::from_digits_array;
use euler::algorithm::combinatorics::permutations_with;
use euler::algorithm::long::from_digits_index;
use euler::algorithm::prime::generator_trial_division;
use euler::Solver;

// The number, 1406357289, is a 0 to 9 pandigital number because it is made up of each of the digits 0 to 9 in some order, but it also has a rather interesting sub-string divisibility property.
// Let d(1) be the 1 st digit, d(2) be the 2 nd digit, and so on. In this way, we note the following:
// d(2) d(3) d(4) = 406 is divisible by 2
// d(3) d(4) d(5) = 063 is divisible by 3
// d(4) d(5) d(6) = 635 is divisible by 5
// d(5) d(6) d(7) = 357 is divisible by 7
// d(6) d(7) d(8) = 572 is divisible by 11
// d(7) d(8) d(9) = 728 is divisible by 13
// d(8) d(9) d(10) = 289 is divisible by 17
// Find the sum of all 0 to 9 pandigital numbers with this property.

const DIM: usize = 3;

pub struct Solver043 {
    pub n: isize,
}

impl Default for Solver043 {
    fn default() -> Self {
        Solver043 { n: 9 }
    }
}

impl Solver for Solver043 {
    fn solve(&self) -> isize {
        let primes = generator_trial_division().take(self.n as usize - DIM + 1).collect::<Vec<_>>();
        let predicate = |d: &[_]| (1..=primes.len()).rev().find(|&n| from_digits_index(d, n, n + DIM) % primes[n - 1] != 0).map(|n| n as isize).xor(Some(from_digits_array(d)));
        permutations_with(0, self.n, predicate).sum()
    }
}
