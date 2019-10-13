// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::long::{int_log_10, pow_10};
use euler::Solver;

// It is possible to show that the square root of two can be expressed as an infinite continued fraction.
//
// sqrt(2) = 1 + 1 / (2 + 1 / (2 + 1 / (2 + ... )
//
// By expanding this for the first four iterations, we get:
//
// 1 + (1 / 2) = 3/2 = 1.5
// 1 + (1 / (2 + 1 / 2) = 7/5 = 1.4
// 1 + (1 / (2 + 1 / (2 + 1 / 2) = 17/12 = 1.41666...
// 1 + (1 / (2 + 1 / (2 + 1 / (2 + 1 / 2) = 41/29 = 1.41379...
//
// The next three expansions are 99/70, 239/169, and 577/408, but the eighth expansion, 1393/985, is the first example where the number of digits in the numerator exceeds the number of digits in the denominator.
//
// In the first one-thousand expansions, how many fractions contain a numerator with more digits than the denominator?

pub struct Solver057 {
    pub n: isize
}

impl Default for Solver057 {
    fn default() -> Self {
        Solver057 { n: 1000 }
    }
}

impl Solver for Solver057 {
    fn solve(&self) -> isize {
        continued_root_two().take(self.n as _).filter(|&(n, d)| int_log_10(n) > int_log_10(d)).count() as _
    }
}

// --- //

struct ContinuedRootTwo {
    n: isize,
    d: isize,
}

fn continued_root_two() -> ContinuedRootTwo {
    ContinuedRootTwo { n: 1, d: 1 }
}

impl Iterator for ContinuedRootTwo {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        // Keeping at most 16 digits, to prevent overflows. Perfectly fine in the context of this problem.
        if self.n > pow_10(16) {
            self.n /= 10;
            self.d /= 10;
        }
        self.d += self.n;
        self.n = (2 * self.d) - self.n;
        Some((self.n, self.d))
    }
}
