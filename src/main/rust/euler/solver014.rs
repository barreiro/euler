// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::long::is_even;
use euler::Solver;

// The following iterative sequence is defined for the set of positive integers: n → n/2 (n is even) n → 3n + 1 (n is odd)
//
// Using the rule above and starting with 13, we generate the following sequence: 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
// It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms.
// Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.
//
// Which starting number, under one million, produces the longest chain?
//
// NOTE: Once the chain starts the terms are allowed to go above one million.

pub struct Solver014 {
    pub n: isize
}

impl Default for Solver014 {
    fn default() -> Self {
        Solver014 { n: 1_000_000 }
    }
}

impl Solver for Solver014 {
    fn solve(&self) -> isize {
        // floor i is an odd number 2/3 of self.n
        let (mut collatz, floor) = (collatz_memoize(self.n), self.n * 2 / 3 - (self.n * 2 / 3) % 2 - 1);
        (floor..self.n).step_by(2).max_by_key(|&x| collatz.length(x)).unwrap()
    }
}

// --- //

struct CollatzMemoize {
    cache: Vec<isize>
}

fn collatz_memoize(size: isize) -> CollatzMemoize {
    let mut cache = vec![0; size as _];
    cache[1] = 1;
    CollatzMemoize { cache }
}

impl CollatzMemoize {
    fn length(&mut self, value: isize) -> isize {
        let (v, size) = (value as usize, self.cache.len());
        if v < size && self.cache[v] != 0 {
            return self.cache[v];
        }

        let collatz = if is_even(value) { 1 + self.length(value >> 1) } else { 2 + self.length((value * 3 + 1) >> 1) };

        if v < size {
            self.cache[v] = collatz;
        }
        collatz
    }
}
