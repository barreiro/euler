// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::long::last_digits;
use euler::Solver;

// The series, 1^1 + 2^2 + 3^3 + ... + 10^10 = 10405071317.
// Find the last ten digits of the series, 1^1 + 2^2 + 3^3 + ... + 1000^1000

const N_DIGITS: isize = 10;

pub struct Solver048 {
    pub n: isize
}

impl Default for Solver048 {
    fn default() -> Self {
        Solver048 { n: 1000 }
    }
}

impl Solver for Solver048 {
    fn solve(&self) -> isize {
        let pow_digits = |i| (1..i).fold(i, |product, _| last_digits(product * i, N_DIGITS));
        last_digits((1..=self.n).map(pow_digits).sum(), N_DIGITS)
    }
}
