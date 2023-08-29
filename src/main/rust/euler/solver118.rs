// COPYRIGHT (C) 2023 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::collections::HashMap;

use algorithm::cast::Cast;
use algorithm::combinatorics::{partitions_of_set, permutations_of_set_with};
use algorithm::digits::{DEFAULT_RADIX, Digit, from_raw_digits};
use algorithm::prime::{miller_rabin, prime_sieve, primes_up_to};
use algorithm::root::{pow_10, square_u64};
use Solver;

const SEARCH_THRESHOLD: u64 = pow_10(5);
const SIEVE_THRESHOLD: u64 = square_u64(SEARCH_THRESHOLD);

/// Using all of the digits `1` through `9` and concatenating them freely to form decimal integers, different sets can be formed.
///
/// Interestingly with the set `{ 2, 5, 47, 89, 631 }`, all of the elements belonging to it are prime.
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
    fn solve(&self) -> i64 {
        // cache some primes to speed up primality test
        let primes = primes_up_to(SEARCH_THRESHOLD).collect::<Vec<_>>();
        let prime_test = |value| if value < SEARCH_THRESHOLD { primes.binary_search(&value).is_ok() } else if value < SIEVE_THRESHOLD { prime_sieve(value, &primes) } else { miller_rabin(value) };

        // calculate the number of primes that a partition generates by multiplying the number of primes in permutations of each partition element
        let (mut cache, prime_permutations) = (HashMap::new(), |p: &Vec<Digit>| permutations_of_set_with(p.clone(), |permutation| prime_test(from_raw_digits(permutation)).then_some(())).count());
        partitions_of_set(&(1..=self.n).collect::<Vec<_>>()).map(|partition| partition.iter().map(|p| {
            if cache.contains_key(p) { cache[p] } else { cache.entry(p.clone()).or_insert_with(|| prime_permutations(p)).to_owned() }.as_i64()
        }).product::<i64>()).sum()
    }
}
