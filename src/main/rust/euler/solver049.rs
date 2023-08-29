// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::collections::HashMap;

use algorithm::cast::Cast;
use algorithm::digits::Digits;
use algorithm::filter::less_than_u64;
use algorithm::prime::primes_up_to;
use algorithm::root::pow_10;
use algorithm::vec::array_concatenation;
use Solver;

const SEQ: usize = 1;

/// The arithmetic sequence, `1487`, `4817`, `8147`, in which each of the terms increases by `3330`, is unusual in two ways:
/// (i) each of the three terms are prime, and, (ii) each of the `4-digit` numbers are permutations of one another.
///
/// There are no arithmetic sequences made up of three `1-`, `2-`, or `3-digit` primes, exhibiting this property, but there is one other `4-digit` increasing sequence.
/// What `12-digit` number do you form by concatenating the three terms in this sequence?
pub struct Solver049 {
    pub n: u64,
}

impl Default for Solver049 {
    fn default() -> Self {
        Self { n: 4 }
    }
}

impl Solver for Solver049 {
    fn problem_name(&self) -> &str { "Prime permutations" }

    fn solve(&self) -> i64 {
        // group together primes based on their permutation --- using their sorted digits as the key on a map
        let mut grouped_primes = HashMap::with_capacity(pow_10(self.n).as_usize());
        primes_up_to(pow_10(self.n)).skip_while(less_than_u64(pow_10(self.n - 1))).for_each(|prime| {
            let digits = Digits::from(prime);
            if !digits.contains(&0) {
                grouped_primes.entry(digits.to_fingerprint()).or_insert_with(Vec::new).push(prime);
            }
        });
        let mut permutations = grouped_primes.values().filter(|&p| p.len() >= 3).collect::<Vec<_>>();
        permutations.sort_unstable();

        // given a list of primes, finds if the average of two others is in the list as well, then convert to digits an concatenate
        let predicate = |&p: &&Vec<_>| (0..p.len() - 2).find_map(|i| (i + 2..p.len()).find_map(|j| p.binary_search(&((p[i] + p[j]) / 2)).ok().map(|k| array_concatenation(&[p[j], p[k], p[i]]))));

        permutations.iter().filter_map(predicate).nth(SEQ).as_i64()
    }
}
