// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::collections::HashSet;

use algorithm::cast::Cast;
use algorithm::digits::{Digits, digits_iter, from_raw_digits};
use Solver;

const TARGET: usize = 60;
const FACTORIAL_CACHE: &[usize] = &[1, 1, 2, 6, 24, 120, 720, 5_040, 40_320, 362_880];

/// The number 145 is well known for the property that the sum of the factorial of its digits is equal to `145`:
///
/// `1! + 4! + 5! = 1 + 24 + 120 = 145`
///
/// Perhaps less well known is `169`, in that it produces the longest chain of numbers that link back to `169`; it turns out that there are only three such loops that exist:
///
/// `169 → 363601 → 1454 → 169`
/// `871 → 45361 → 871`
/// `872 → 45362 → 872`
///
/// It is not difficult to prove that *EVERY* starting number will eventually get stuck in a loop. For example,
///
/// `69 → 363600 → 1454 → 169 → 363601 (→ 1454)`
/// `78 → 45360 → 871 → 45361 (→ 871)`
/// `540 → 145 (→ 145)`
///
/// Starting with `69` produces a chain of five non-repeating terms, but the longest non-repeating chain with a starting number below one million is sixty terms.
///
/// How many chains, with a starting number below one million, contain exactly sixty non-repeating terms?
pub struct Solver074 {
    pub n: usize,
}

impl Default for Solver074 {
    fn default() -> Self {
        Self { n: 1_000_000 }
    }
}

impl Solver for Solver074 {
    fn solve(&self) -> i64 {
        let factorial_cycle_len = |mut l: usize, cache: &[_]| {
            let mut set = HashSet::new(); // HashSet is much faster than BitSet here
            loop {
                if let Some(Some(c)) = cache.get(l) {
                    break c + set.len();
                } else if !set.insert(l.as_u64()) {
                    break set.len();
                }
                l = digits_iter(l.as_u64()).map(|d| FACTORIAL_CACHE[d.as_usize()]).sum();
            }
        };

        // permutations starting with 0 are discarded as 0! is 1
        let (predicate, mut cache) = (|d: &[_]| d.last().map_or(false, |&l| l != 0).then(|| from_raw_digits(d)), vec![None; self.n]);

        (1..self.n).filter(|&i| {
            if cache[i].is_none() {
                cache[i] = Some(factorial_cycle_len(i, &cache));
                Digits::from(i.as_u64()).permutations_with(predicate).for_each(|permutation| cache[permutation.as_usize()] = cache[i]);
            }
            cache[i] == Some(TARGET)
        }).count().as_i64()
    }
}
