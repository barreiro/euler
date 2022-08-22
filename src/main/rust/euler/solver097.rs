// COPYRIGHT (C) 2022 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::long::{pow_10, power_modulo, safe_mul_mod};
use euler::Solver;

// The first known prime found to exceed one million digits was discovered in 1999, and is a Mersenne prime of the form 2^6972593 − 1; it contains exactly 2,098,960 digits.
// Subsequently other Mersenne primes, of the form 2^p−1, have been found which contain more digits.
//
// However, in 2004 there was found a massive non-Mersenne prime which contains 2,357,207 digits: 28433 * 2^7830457 + 1.
//
// Find the last ten digits of this prime number.

const FACTOR: isize = 28_433;
const EXP: isize = 7_830_457;

pub struct Solver097 {
    pub n: isize,
}

impl Default for Solver097 {
    fn default() -> Self {
        Solver097 { n: 10 }
    }
}

impl Solver for Solver097 {
    fn solve(&self) -> isize {
        // let (mut b, modulo, step) = (FACTOR, pow_10(self.n), 10);
        // for _ in 0..EXP / step {
        //     b <<= step;
        //     b %= modulo;
        // }
        // b <<= EXP % step;
        // b %= modulo;
        // b + 1

        safe_mul_mod(FACTOR, power_modulo(2, EXP, pow_10(self.n)), pow_10(self.n)) + 1
    }
}
