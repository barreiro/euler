// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::collections::HashSet;

use euler::algorithm::combinatorics::permutations_of_set_with;
use euler::algorithm::long::{from_digits_index, to_digits};
use euler::Solver;

// The number 145 is well known for the property that the sum of the factorial of its digits is equal to 145:
//
// 1! + 4! + 5! = 1 + 24 + 120 = 145
//
// Perhaps less well known is 169, in that it produces the longest chain of numbers that link back to 169; it turns out that there are only three such loops that exist:
//
// 169 → 363601 → 1454 → 169
// 871 → 45361 → 871
// 872 → 45362 → 872
//
// It is not difficult to prove that EVERY starting number will eventually get stuck in a loop. For example,
//
// 69 → 363600 → 1454 → 169 → 363601 (→ 1454)
// 78 → 45360 → 871 → 45361 (→ 871)
// 540 → 145 (→ 145)
//
// Starting with 69 produces a chain of five non-repeating terms, but the longest non-repeating chain with a starting number below one million is sixty terms.
//
// How many chains, with a starting number below one million, contain exactly sixty non-repeating terms?

const TARGET: usize = 60;
const FACTORIAL_CACHE: &[usize] = &[1, 1, 2, 6, 24, 120, 720, 5_040, 40_320, 362_880];

pub struct Solver074 {
    pub n: isize
}

impl Default for Solver074 {
    fn default() -> Self {
        Solver074 { n: 1_000_000 }
    }
}

impl Solver for Solver074 {
    fn solve(&self) -> isize {
        let factorial_cycle_len = |mut l, cache: &[_]| {
            let mut set = HashSet::new();
            loop {
                if *cache.get(l).unwrap_or(&0) != 0 {
                    break cache[l] + set.len();
                } else if !set.insert(l) {
                    break set.len();
                }
                l = to_digits(l as _).iter().map(|&d| FACTORIAL_CACHE[d as usize]).sum();
            }
        };

        // permutations starting with 0 are discarded as 0! is 1
        let (predicate, mut cache) = (|d: &[_]| if d[d.len() - 1] != 0 { Some(from_digits_index(d, 0, d.len())) } else { None }, vec![0; self.n as _]);

        // iterate in reverse order due to the nature of the permutations_of() iterator
        (1..self.n as _).rev().filter(|&i| {
            if cache[i] == 0 {
                cache[i] = factorial_cycle_len(i, &cache);
                permutations_of_set_with(to_digits(i as _), predicate).for_each(|permutation| cache[permutation as usize] = cache[i]);
            }
            cache[i] == TARGET
        }).count() as _
    }
}
