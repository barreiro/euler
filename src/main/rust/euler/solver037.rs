// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::bit::BitSet;
use euler::algorithm::long::DEFAULT_RADIX;
use euler::algorithm::prime::generator_trial_division;
use euler::Solver;

// The number 3797 has an interesting property. Being prime itself, it is possible to continuously remove digits from left to right, and remain prime at each stage: 3797, 797, 97, and 7. Similarly we can work from right to left: 3797, 379, 37, and 3.
// Find the sum of the only eleven primes that are both truncatable from left to right and right to left.
// NOTE: 2, 3, 5, and 7 are not considered to be truncatable primes.

pub struct Solver037 {
    pub n: isize,
}

impl Default for Solver037 {
    fn default() -> Self {
        Solver037 { n: 11 }
    }
}

impl Solver for Solver037 {
    fn solve(&self) -> isize {
        let (generator, mut set) = (generator_trial_division().skip(4), [2, 3, 5, 7].iter().collect::<BitSet>());
        generator.filter(|&p| set.insert(p) && is_truncatable_right(p, &set) && is_truncatable_left(p, &set)).take(self.n as _).sum()
    }
}

fn is_truncatable_right(p: isize, set: &BitSet) -> bool {
    let mut l = p / DEFAULT_RADIX;
    while l > 0 {
        if !set.contains(l) {
            return false;
        }
        l /= DEFAULT_RADIX;
    }
    true
}

fn is_truncatable_left(p: isize, set: &BitSet) -> bool {
    let (mut l, mut n) = (p % DEFAULT_RADIX, DEFAULT_RADIX);
    while l < p {
        if !set.contains(l) {
            return false;
        }
        n *= DEFAULT_RADIX;
        l = p % n;
    }
    true
}


