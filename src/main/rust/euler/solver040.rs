// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::cast::Cast;
use algorithm::digits::{DEFAULT_RADIX, nth_digit};
use algorithm::long::GetAndIncrement;
use algorithm::root::{pow_10, pow_u64};
use Solver;

const RADIX: u64 = DEFAULT_RADIX as u64;

/// An irrational decimal fraction is created by concatenating the positive integers:
/// `0.123456789101112131415161718192021...`
///
/// It can be seen that the `12th digit` of the fractional part is `1`.
///
/// If `dn` represents the `nth digit` of the fractional part, find the value of the following expression.
/// ```
/// d1 × d10 × d100 × d1000 × d10000 × d100000 × d1000000
/// ```
pub struct Solver040 {
    pub n: u64,
}

impl Default for Solver040 {
    fn default() -> Self {
        Self { n: 7 }
    }
}

impl Solver for Solver040 {
    fn problem_name(&self) -> &str { "Champernowne's constant" }

    fn solve(&self) -> i64 {
        let ledge = |position| {
            // ledges are the places where concatenation starts to have bigger integers. to each there is a corresponding integer length.
            let (mut length, mut ledge) = (1, 0);
            loop {
                let previous = ledge;
                ledge += (RADIX - 1) * length * pow_u64(RADIX, length.get_and_increment() - 1);
                if position <= ledge {
                    return (length - 1, position - 1 - previous);
                }
            }
        };

        let d_function = |position| {
            // compute the size of the integer and it's offset from the ledge. from there calculate the number and the index of the digit.
            let (length, offset) = ledge(position);
            nth_digit(pow_10(length - 1) + offset / length, offset % length)
        };

        (0..self.n).map(pow_10).map(d_function).product::<u64>().as_i64()
    }
}
