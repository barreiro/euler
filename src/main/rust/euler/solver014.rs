// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use crate::algorithm::cast::Cast;
use crate::algorithm::filter::is_even_u64;
use crate::Solver;

/// The following iterative sequence is defined for the set of positive integers: `n → n/2` (n is even) `n → 3n + 1` (n is odd)
///
/// Using the rule above and starting with `13`, we generate the following sequence: `13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1`
/// It can be seen that this sequence (starting at `13` and finishing at `1`) contains `10` terms.
/// Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at `1`.
///
/// Which starting number, under one million, produces the longest chain?
///
/// NOTE: Once the chain starts the terms are allowed to go above one million.
pub struct Solver014 {
    pub n: u64,
}

impl Default for Solver014 {
    fn default() -> Self {
        Self { n: 1_000_000 }
    }
}

impl Solver for Solver014 {
    fn problem_name(&self) -> &str { "Longest collatz sequence" }

    fn solve(&self) -> i64 {
        // floor `i` is an odd number `2/3` of `self.n`
        let (mut collatz, floor) = (collatz_memoize(self.n), self.n * 2 / 3 - (self.n * 2 / 3) % 2 - 1);
        (floor..self.n).step_by(2).max_by_key(|&x| collatz.length(x)).as_i64()
    }
}

// --- //

struct CollatzMemoize {
    cache: Vec<Option<usize>>,
}

fn collatz_memoize(size: u64) -> CollatzMemoize {
    let mut cache = vec![None; size.as_usize()];
    cache[1] = Some(1);
    CollatzMemoize { cache }
}

impl CollatzMemoize {
    fn length(&mut self, value: u64) -> usize {
        if let Some(Some(collatz)) = self.cache.get(value.as_usize()) {
            *collatz
        } else {
            let collatz = if is_even_u64(&value) { 1 + self.length(value >> 1) } else { 2 + self.length((value * 3 + 1) >> 1) };
            self.cache.get_mut(value.as_usize()).iter_mut().for_each(|cache| **cache = Some(collatz));
            collatz
        }
    }
}
