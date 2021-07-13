// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::long::power_modulo;
use euler::algorithm::prime::{prime_factors, descending_primes};
use euler::Solver;

// A unit fraction contains 1 in the numerator. The decimal representation of the unit fractions with denominators 2 to 10 are given:
// 1/2  =  0.5
// 1/3  =  0.(3)
// 1/4  =  0.25
// 1/5  =  0.2
// 1/6  =  0.1(6)
// 1/7  =  0.(142857)
// 1/8  =  0.125
// 1/9  =  0.(1)
// 1/10 =  0.1
// Where 0.1(6) means 0.166666..., and has a 1-digit recurring cycle. It can be seen that 1/7 has a 6-digit recurring cycle.
// Find the value of d < 1000 for which 1/d contains the longest recurring cycle in its decimal fraction part.

pub struct Solver026 {
    pub n: isize,
}

impl Default for Solver026 {
    fn default() -> Self {
        Solver026 { n: 1000 }
    }
}

impl Solver for Solver026 {
    fn solve(&self) -> isize {
        // For primes: if 10 is a primitive root modulo p, the recurring cycle is equal to p − 1; if not is a factor of p − 1 (thus smaller)
        let is_prime_root_ten = |p: &_| prime_factors(p - 1).keys().all(|&f| power_modulo(10, (p - 1) / f, *p) != 1);

        descending_primes(self.n).find(is_prime_root_ten).unwrap()
    }
}
