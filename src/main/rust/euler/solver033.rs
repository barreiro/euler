// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::Solver;

// The fraction 49/98 is a curious fraction, as an inexperienced mathematician in attempting to simplify it may incorrectly believe that 49/98 = 4/8, which is correct, is obtained by cancelling the 9s.
// We shall consider fractions like, 30/50 = 3/5, to be trivial examples.
// There are exactly four non-trivial examples of this type of fraction, less than one in value, and containing two digits in the numerator and denominator.
// If the product of these four fractions is given in its lowest common terms, find the value of the denominator.

const BASE: isize = 10;

pub struct Solver033 {
    pub n: isize,
}

impl Default for Solver033 {
    fn default() -> Self {
        Solver033 { n: 100 }
    }
}

impl Solver for Solver033 {
    fn solve(&self) -> isize {
        let mut product = 1;
        for denominator in 1..self.n {
            for numerator in 1..denominator {
                for radix in 1..BASE {
                    if naive_cancellation(numerator, denominator, radix) {
                        product = product * denominator / numerator;
                    }
                }
            }
        }
        product
    }
}

// This is a bit over-optimized!
// What happens here is that we first check if the end of the numerator is equal to the start of the denominator.
// We then check to see if the fractions match, juggling a bit with the terms to avoid double calculation.
//
// => naiveNumerator = numerator / 10, naiveDenominator = denominator - radix * 10
// => numerator / denominator == naiveNumerator / naiveDenominator
fn naive_cancellation(n: isize, d: isize, r: isize) -> bool {
    n % 10 == r && d > r * 10 && n * (d - r * BASE) == d * (n / BASE)
}

