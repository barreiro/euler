// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::cast::Cast;
use algorithm::combinatorics::permutations_of_digits_with;
use algorithm::digits::from_raw_digits;
use algorithm::filter::is_prime;
use Solver;

/// We shall say that an `n-digit` number is pandigital if it makes use of all the digits `1` to `n` exactly once.
/// For example, `2143` is a `4-digit` pandigital and is also prime.
/// What is the largest `n-digit` pandigital prime that exists?
pub struct Solver041 {
    pub n: u8,
}

impl Default for Solver041 {
    fn default() -> Self {
        Self { n: 9 }
    }
}

impl Solver for Solver041 {
    fn solve(&self) -> i64 {
        // assume the largest prime also start with the biggest digit
        let predicate = |d: &[u8]| if *d.last().unwrap() as usize != d.len() || d.first().unwrap() % 2 == 0 { None } else { Some(from_raw_digits(d)).filter(is_prime) };

        // if the sum of the digits of the permutation is multiple of three, all permutations are multiple of three as well
        (1..=self.n).rev().filter(|&n| n % 3 != 0).find_map(|n| permutations_of_digits_with(1, n, predicate).max()).as_i64()
    }
}
