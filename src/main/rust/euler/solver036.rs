// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::combinatorics::palindrome_array;
use euler::algorithm::long::{from_digits, is_palindrome_radix};
use euler::Solver;

// The decimal number, 585 = 10010010012 (binary), is palindromic in both bases.
// Find the sum of all numbers, less than one million, which are palindromic in base 10 and base 2.
// (Please note that the palindromic number, in either base, may not include leading zeros.)

pub struct Solver036 {
    pub n: isize,
}

impl Default for Solver036 {
    fn default() -> Self {
        Solver036 {
            n: 1000000
        }
    }
}

impl Solver for Solver036 {
    fn solve(&self) -> isize {
        palindrome_array().map(|array| from_digits(&array)).take_while(|&p| p < self.n).filter(|&p| is_palindrome_radix(p, 2)).sum()
    }
}
