// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

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
        let mut collatz = collatz_memoize(self.n as _);
        (3..self.n).step_by(2).max_by(|&x, &y| collatz.length(x).cmp(&collatz.length(y))).unwrap()
    }
}

// --- //

struct CollatzMemoize {
    size: usize,
    cache: Vec<isize>,
}

fn collatz_memoize(size: usize) -> CollatzMemoize {
    let mut cache = vec![0; size];
    cache[1] = 1;
    CollatzMemoize { size, cache }
}

impl CollatzMemoize {
    fn length(&mut self, value: isize) -> isize {
        let i = value as usize;
        if i < self.size && self.cache[i] != 0 {
            return self.cache[i];
        }
        let collatz = 1 + self.length(if value & 1 == 0 { value / 2 } else { value * 3 + 1 });

        if i < self.size {
            self.cache[i] = collatz;
        }
        collatz
    }
}
