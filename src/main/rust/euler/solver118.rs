// COPYRIGHT (C) 2023 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::collections::HashMap;

use crate::algorithm::cast::Cast;
use crate::algorithm::combinatorics::{partitions_of_set, permutations_of_set_with};
use crate::algorithm::digits::{from_raw_digits, Digit, DEFAULT_RADIX};
use crate::algorithm::prime::PrimeTestWithCache;
use crate::Solver;

/// Using all the digits `1` through `9` and concatenating them freely to form decimal integers, different sets can be formed.
///
/// Interestingly with the set `{ 2, 5, 47, 89, 631 }`, all the elements belonging to it are prime.
///
/// How many distinct sets containing each of the digits one through nine exactly once contain only prime elements?
pub struct Solver118 {
    pub n: Digit,
}

impl Default for Solver118 {
    fn default() -> Self {
        Self { n: DEFAULT_RADIX - 1 }
    }
}

impl Solver for Solver118 {
    fn problem_name(&self) -> &str { "Pandigital prime sets" }

    fn solve(&self) -> i64 {
        let tester = PrimeTestWithCache::default();

        // calculate the number of primes that a partition generates by multiplying the number of primes in permutations of each partition element
        let (mut cache, prime_permutations) = (HashMap::new(), |p: &Vec<Digit>| permutations_of_set_with(p.clone(), |permutation| tester.is_prime(from_raw_digits(permutation)).then_some(())).count());
        partitions_of_set(&(1..=self.n).collect::<Vec<_>>()).map(|partition| partition.iter().map(|p| {
            if cache.contains_key(p) { cache[p] } else { cache.entry(p.clone()).or_insert_with(|| prime_permutations(p)).to_owned() }.as_i64()
        }).product::<i64>()).sum()
    }
}
