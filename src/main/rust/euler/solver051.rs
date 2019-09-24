// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::long::{from_digits, pow_10, to_digits};
use euler::algorithm::prime::{generator_trial_division, miller_rabin};
use euler::Solver;

// By replacing the 1st digit of the 2-digit number *3, it turns out that six of the nine possible values: 13, 23, 43, 53, 73, and 83, are all prime.
// By replacing the 3rd and 4th digits of 56**3 with the same digit, this 5-digit number is the first example having seven primes among the ten generated numbers, yielding the family: 56003, 56113, 56333, 56443, 56663, 56773, and 56993.
// Consequently 56003, being the first member of this family, is the smallest prime with this property.
// Find the smallest prime which, by replacing part of the number (not necessarily adjacent digits) with the same digit, is part of an eight prime value family.

pub struct Solver051 {
    pub n: isize
}

impl Default for Solver051 {
    fn default() -> Self {
        Solver051 { n: 8 }
    }
}

impl Solver for Solver051 {
    fn solve(&self) -> isize {
        // n repeated digits are required in order to produce a family of size 5 + n
        let (required_repetitions, small) = (self.n as usize - 5, |&p: &_| p < pow_10(self.n - 5));

        let predicate = |&p: &_| {
            let digits = to_digits(p);

            // find a digit that is repeated the required amount of times and replace it's occurrences
            (0..=9 - self.n).find(|i| digits.iter().skip(1).filter(|&d| d == i).count() >= required_repetitions).map_or(false, |repeated| {
                // NOTE: the digit can be repeated more times than required. To be correct should try only subsets of the repeated digit in that case.
                digits[0] != repeated && (0..=9).filter(|&substitute| {
                    let candidate = from_digits(digits.iter().map(|&d| if d == repeated { substitute } else { d }).collect());
                    candidate < p || !miller_rabin(candidate)
                    // count the elements *not* in the family, to a allow an early return
                }).nth(1 + 9 - self.n as usize).is_none()
            })
        };

        generator_trial_division().skip_while(small).find(predicate).unwrap()
    }
}
