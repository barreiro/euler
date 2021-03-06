// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::Solver;

// The Fibonacci sequence is defined by the recurrence relation: Fn = Fn−1 + Fn−2, where F1 = 1 and F2 = 1.
// Hence the first 12 terms will be:
// F1 = 1
// F2 = 1
// F3 = 2
// F4 = 3
// F5 = 5
// F6 = 8
// F7 = 13
// F8 = 21
// F9 = 34
// F10 = 55
// F11 = 89
// F12 = 144
// The 12th term, F12, is the first term to contain three digits.
// What is the first term in the Fibonacci sequence to contain 1000 digits?

pub struct Solver025 {
    pub n: isize,
}

impl Default for Solver025 {
    fn default() -> Self {
        Solver025 { n: 1000 }
    }
}

impl Solver for Solver025 {
    fn solve(&self) -> isize {
        let (log_root_5, log_phi) = (5_f64.sqrt().log10(), ((1.0 + 5_f64.sqrt()) / 2.0).log10());

        // Using the logarithm (base 10) of Binet's Formula (approximation)
        ((self.n as f64 - 1.0 + log_root_5) / log_phi).ceil() as _
    }
}
