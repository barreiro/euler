// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::prime::{generator_trial_division, miller_rabin};
use euler::Solver;
use euler::algorithm::long::DEFAULT_RADIX;

// The number 3797 has an interesting property. Being prime itself, it is possible to continuously remove digits from left to right, and remain prime at each stage: 3797, 797, 97, and 7. Similarly we can work from right to left: 3797, 379, 37, and 3.
// Find the sum of the only eleven primes that are both truncatable from left to right and right to left.
// NOTE: 2, 3, 5, and 7 are not considered to be truncatable primes.

pub struct Solver037 {
    pub n: isize,
}

impl Default for Solver037 {
    fn default() -> Self {
        Solver037 {
            n: 11
        }
    }
}

impl Solver for Solver037 {
    fn solve(&self) -> isize {
        generator_trial_division().skip(4).filter(|&p| is_truncatable_left(p) && is_truncatable_right(p)).take(self.n as usize).sum()
    }
}

fn is_truncatable_right(p: isize) -> bool {
    let mut l = p / DEFAULT_RADIX;
    while l > 0 {
        if !miller_rabin(l) {
            return false;
        }
        l /= DEFAULT_RADIX;
    }
    return true;
}

fn is_truncatable_left(p: isize) -> bool {
    let (mut l, mut n) = (p % DEFAULT_RADIX, DEFAULT_RADIX);
    while l < p {
        if !miller_rabin(l) {
            return false;
        }
        n *= DEFAULT_RADIX;
        l = p % n;
    }
    return true;
}


