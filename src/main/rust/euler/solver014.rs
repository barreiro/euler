// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::alloc::{alloc_zeroed, dealloc, Layout};
use std::mem;

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
        Solver014 { n: 1000000 }
    }
}

impl Solver for Solver014 {
    fn solve(&self) -> isize {
        let (mut i, mut max, mut max_length, collatz) = (self.n, 1, 1, CollatzMemoize::with_size(self.n));
        while i > 1 {
            i -= 1;
            let collatz_length = collatz.length(i);
            if collatz_length > max_length {
                max_length = collatz_length;
                max = i;
            }
        }
        max
    }
}

// --- //

struct CollatzMemoize {
    size: isize,
    cache: *mut isize,
}

impl CollatzMemoize {
    fn with_size(size: isize) -> Self {
        let cache = unsafe { alloc_zeroed(Layout::from_size_align_unchecked(mem::size_of::<isize>() * size as usize, mem::align_of::<isize>())) as *mut isize };
        let instance = CollatzMemoize { size, cache };
        unsafe { instance.cache.offset(1).write(1) }
        instance
    }

    fn length(&self, i: isize) -> isize {
        if i < self.size {
            let cached = unsafe { self.cache.offset(i).read() };
            if cached != 0 {
                return cached;
            }
        }

        let collatz = self.length(if i & 1 == 0 { i / 2 } else { i * 3 + 1 }) + 1;

        if i < self.size {
            unsafe { self.cache.offset(i).write(collatz) }
        }
        collatz
    }
}

impl Drop for CollatzMemoize {
    fn drop(&mut self) {
        unsafe { dealloc(self.cache as *mut u8, Layout::from_size_align_unchecked(mem::size_of::<isize>() * self.size as usize, mem::align_of::<isize>())) };
    }
}
