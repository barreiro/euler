// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::long::{from_digits_array, to_digits};
use euler::algorithm::prime::{miller_rabin, primes_wheel_up_to};
use euler::Solver;

// The number, 197, is called a circular prime because all rotations of the digits: 197, 971, and 719, are themselves prime.
// There are thirteen such primes below 100: 2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, and 97.
// How many circular primes are there below one million?

pub struct Solver035 {
    pub n: isize,
}

impl Default for Solver035 {
    fn default() -> Self {
        Solver035 { n: 1_000_000 }
    }
}

impl Solver for Solver035 {
    fn solve(&self) -> isize {
        let is_circular_prime = |prime: &isize| {
            let mut digits = to_digits(*prime);
            let len = digits.len();

            // circular primes are only made of the digits 1, 3, 7 and 9
            if digits.iter().any(|&d| d & 1 == 0 || d == 5) && *prime >= 10 {
                return false;
            }

            let rotation = |_| {
                digits.rotate_left(1);
                from_digits_array(&digits)
            };
            (1..len).map(rotation).all(miller_rabin)
        };

        primes_wheel_up_to(self.n).filter(is_circular_prime).count() as _
    }
}
