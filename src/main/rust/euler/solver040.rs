// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::long::{pow_10, nth_digit, DEFAULT_RADIX};
use euler::Solver;

// An irrational decimal fraction is created by concatenating the positive integers:
// 0.123456789101112131415161718192021...
//
// It can be seen that the 12th digit of the fractional part is 1.
// If dn represents the nth digit of the fractional part, find the value of the following expression.
//
// d1 × d10 × d100 × d1000 × d10000 × d100000 × d1000000

pub struct Solver040 {
    pub n: isize,
}

impl Default for Solver040 {
    fn default() -> Self {
        Solver040 {
            n: 7
        }
    }
}

impl Solver for Solver040 {
    fn solve(&self) -> isize {
        let ledge = |position| {
            // Ledges are the places where concatenation starts to have bigger integers. To each there is a corresponding integer length.
            let (mut length, mut ledge) = (1, 0);
            loop {
                let previous = ledge;
                ledge += (DEFAULT_RADIX - 1) * length * pow_10(length - 1);
                length += 1;
                if position <= ledge {
                    return (length - 1, position - 1 - previous);
                }
            }
        };

        let d = |position| {
            // Compute the size of the integer and it's offset from the ledge. From there calculate the number and the index of the digit.
            let (length, offset) = ledge(position);
            nth_digit(pow_10(length - 1) + offset / length, offset % length + 1)
        };

        (0..self.n).map(|n| pow_10(n)).map(|n| d(n)).product()
    }
}
