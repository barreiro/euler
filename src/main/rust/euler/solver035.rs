// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::cast::Cast;
use algorithm::digits::Digits;
use algorithm::prime::{miller_rabin, primes_wheel_up_to};
use Solver;

/// The number `197` is called a circular prime because all rotations of the digits: `197, 971, and 719` are themselves prime.
/// There are thirteen such primes below `100`: `2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, and 97`.
/// How many circular primes are there below one million?
pub struct Solver035 {
    pub n: u64,
}

impl Default for Solver035 {
    fn default() -> Self {
        Self { n: 1_000_000 }
    }
}

impl Solver for Solver035 {
    fn solve(&self) -> i64 {
        let is_circular_prime = |&prime: &_| {
            let mut digits = Digits::from(prime);

            // circular primes are only made of the digits 1, 3, 7 and 9
            if digits.iter().any(|&d| d % 2 == 0 || d == 5) && prime >= 10 {
                return false;
            }

            (1..digits.len()).map(|_| {
                digits.rotate_left();
                digits.value()
            }).all(miller_rabin)
        };

        primes_wheel_up_to(self.n).filter(is_circular_prime).count().as_i64()
    }
}
