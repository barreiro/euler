// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use crate::algorithm::cast::Cast;
use crate::Solver;

/// The Fibonacci sequence is defined by the recurrence relation: `Fn = Fn−1 + Fn−2`, where `F1 = 1` and `F2 = 1`.
/// Hence, the first `12` terms will be:
/// ```
/// F1 = 1
/// F2 = 1
/// F3 = 2
/// F4 = 3
/// F5 = 5
/// F6 = 8
/// F7 = 13
/// F8 = 21
/// F9 = 34
/// F10 = 55
/// F11 = 89
/// F12 = 144
/// ```
/// The `12th` term, `F12`, is the first term to contain three digits.
///
/// What is the first term in the Fibonacci sequence to contain `1000` digits?
pub struct Solver025 {
    pub n: u32,
}

impl Default for Solver025 {
    fn default() -> Self {
        Self { n: 1000 }
    }
}

impl Solver for Solver025 {
    fn problem_name(&self) -> &str { "1000-digit fibonacci number" }

    fn solve(&self) -> i64 {
        // from https://r-knott.surrey.ac.uk/Fibonacci/fibFormula.html#section2.3
        // using the logarithm (base 10) of Binet's Formula (approximation)
        let (log_root_5, log_phi) = (5_f64.log10() / 2.0, ((1.0 + 5_f64.sqrt()) / 2.0).log10());
        ((f64::from(self.n) - 1.0 + log_root_5) / log_phi).ceil().as_i64()
    }
}
