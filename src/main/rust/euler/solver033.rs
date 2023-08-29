// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::cast::Cast;
use algorithm::digits::DEFAULT_RADIX;
use Solver;

const BASE: u64 = DEFAULT_RADIX as u64;

/// The fraction `49/98` is a curious fraction, as an inexperienced mathematician in attempting to simplify it may incorrectly believe that `49/98 = 4/8`, which is correct, is obtained by cancelling the `9`s.
///
/// We shall consider fractions like, `30/50 = 3/5`, to be trivial examples.
/// 
/// There are exactly four non-trivial examples of this type of fraction, less than one in value, and containing two digits in the numerator and denominator.
///
/// If the product of these four fractions is given in its lowest common terms, find the value of the denominator.
pub struct Solver033 {
    pub n: u64,
}

impl Default for Solver033 {
    fn default() -> Self {
        Self { n: 100 }
    }
}

impl Solver for Solver033 {
    fn problem_name(&self) -> &str { "Digit cancelling fractions" }

    fn solve(&self) -> i64 {
        let mut product = 1;
        (1..self.n).for_each(|denominator| (1..denominator).for_each(|numerator| {
            (1..BASE).filter(|&radix| naive_cancellation(numerator, denominator, radix)).for_each(|_| {
                // using the *= operator causes rounding errors
                product = product * denominator / numerator;
            });
        }));
        product.as_i64()
    }
}

// this is a bit over-optimized!
// first check if the end of the numerator is equal to the start of the denominator then check if the fractions match
// juggling a bit with the terms to avoid double calculation.
//
// => naiveNumerator = numerator / 10, naiveDenominator = denominator - radix * 10
// => numerator / denominator == naiveNumerator / naiveDenominator
const fn naive_cancellation(n: u64, d: u64, r: u64) -> bool {
    n % 10 == r && d > r * 10 && n * (d - r * BASE) == d * (n / BASE)
}

