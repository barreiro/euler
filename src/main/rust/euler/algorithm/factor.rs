// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::long::{exact_sqrt, floor_sqrt, is_square};

pub fn has_factor_below(value: isize, roof: isize) -> bool {
    (floor_sqrt(value)..roof).any(|l| value % l == 0 && value / l < roof)
}

pub fn totient(value: isize) -> isize {
    let (mut f, mut n, mut totient) = (2, value, value);
    while f * f <= n {
        if if n <= i32::MAX as _ { n as i32 % f as i32 == 0 } else { n % f == 0 } {
            while n % f == 0 {
                n /= f;
            }
            totient -= totient / f;
        }
        f += 1;
    }
    if n > 1 {
        totient -= totient / n;
    }
    totient
}

pub fn number_of_factors(value: isize) -> isize {
    // need to adjust the number of divisors if the number is a perfect square
    ((proper_factors_of(value).count() as isize) << 1) - if is_square(value) { 1 } else { 0 }
}

// defined according to problem 21: numbers less than n which divide evenly into n
pub fn sum_of_factors(value: isize) -> isize {
    let (sum, (sqrt, rem)) = (proper_factors_of(value).map(|f| f + value / f).sum::<isize>(), exact_sqrt(value));

    // need to adjust the sum if the number is a perfect square
    if rem == 0 { sum - sqrt - value } else { sum - value }
}

pub fn is_abundant(value: isize) -> bool {
    value < sum_of_factors(value)
}

// --- //

/// closure that generates the proper factors, the factors up until the square root (in reverse order)
pub struct ProperFactor {
    value: isize,
    f: isize,
    small: bool,
}

pub fn proper_factors_of(value: isize) -> ProperFactor {
    ProperFactor { value, f: floor_sqrt(value) + 1, small: value <= i32::MAX as isize }
}

impl Iterator for ProperFactor {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.f > 1 {
            self.f -= 1;
            if if self.small { self.value as i32 % self.f as i32 == 0 } else { self.value % self.f == 0 } {
                return Some(self.f);
            }
        }
        None
    }
}