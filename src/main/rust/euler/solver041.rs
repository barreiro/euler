// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use crate::algorithm::cast::Cast;
use crate::algorithm::combinatorics::permutations_of_digits_with;
use crate::algorithm::digits::{Digit, from_raw_digits};
use crate::algorithm::filter::is_prime;
use crate::Solver;

/// We shall say that an `n-digit` number is pandigital if it makes use of all the digits `1` to `n` exactly once.
///
/// For example, `2143` is a `4-digit` pandigital and is also prime.
///
/// What is the largest `n-digit` pandigital prime that exists?
pub struct Solver041 {
    pub n: Digit,
}

impl Default for Solver041 {
    fn default() -> Self {
        Self { n: 9 }
    }
}

impl Solver for Solver041 {
    fn problem_name(&self) -> &str { "Pandigital prime" }

    fn solve(&self) -> i64 {
        // assume the largest prime also start with the biggest digit
        let predicate = |d: &[Digit]| (*d.last().expect("Permutation should not be empty") as usize != d.len() || d.first().expect("Permutation should not be empty") % 2 != 0).then(|| from_raw_digits(d)).filter(is_prime);

        // if the sum of the digits of the permutation is multiple of three, all permutations are multiple of three as well
        (1..=self.n).rev().filter(|&n| n % 3 != 0).find_map(|n| permutations_of_digits_with(1, n, predicate).max()).as_i64()
    }
}
