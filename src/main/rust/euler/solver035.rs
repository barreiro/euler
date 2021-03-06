// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::long::{from_digits_index, int_log_10, to_digits};
use euler::algorithm::prime::{generator_wheel, miller_rabin};
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
        generator_wheel().take_while(|&p| p < self.n).filter(|&p| is_circular_prime(p)).count() as _
    }
}

fn is_circular_prime(prime: isize) -> bool {
    let mut digits = to_digits(prime);

    // circular primes are only made of the digits 1, 3, 7 and 9
    if digits.iter().any(|&d| d & 1 == 0 || d == 5) && prime >= 10 {
        return false;
    }

    let rotation = |_| {
        digits.rotate_left(1);
        from_digits_index(&digits, 0, digits.len())
    };
    (1..int_log_10(prime)).map(rotation).all(miller_rabin)
}
