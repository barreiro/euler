// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::long::{from_digits, int_log_10, to_digits};
use euler::algorithm::prime::{generator_trial_division, miller_rabin};
use euler::Solver;

// The number, 197, is called a circular prime because all rotations of the digits: 197, 971, and 719, are themselves prime.
// There are thirteen such primes below 100: 2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, and 97.
// How many circular primes are there below one million?

pub struct Solver035 {
    pub n: isize,
}

impl Default for Solver035 {
    fn default() -> Self {
        Solver035 {
            n: 1000000
        }
    }
}

impl Solver for Solver035 {
    fn solve(&self) -> isize {
        generator_trial_division().take_while(|&p| p < self.n).filter(|&p| is_circular_prime(p)).count() as isize
    }
}

fn is_circular_prime(prime: isize) -> bool {
    let digits = &mut to_digits(prime);

    // circular primes are only made of the digits 1, 3, 7 and 9
    if prime >= 10 && digits.iter().any(|&d| d != 1 && d != 3 && d != 7 && d != 9) {
        return false;
    }

    let mut next_rotation = || {
        digits.rotate_left(1);
        from_digits(digits)
    };
    (1..int_log_10(prime)).map(|_| next_rotation()).all(|p| miller_rabin(p))
}
