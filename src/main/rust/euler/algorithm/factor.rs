// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::collections::HashMap;
use std::hash::BuildHasher;

use euler::algorithm::long::{floor_sqrt, is_square, pow};

pub fn factor_composition<S: BuildHasher>(factor_map: HashMap<isize, isize, S>) -> isize {
    factor_map.iter().map(|(&base, &exp)| pow(base, exp)).product()
}

pub fn has_factor_below(value: isize, roof: isize) -> bool {
    (floor_sqrt(value)..roof).any(|l| value % l == 0 && value / l < roof)
}

pub fn totient(value: isize) -> isize {
    let (mut f, mut n, mut totient) = (2, value, value);
    while f * f <= n {
        if if n <= i32::max_value() as _ { n as i32 % f as i32 == 0 } else { n % f == 0 } {
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
    let (mut f, mut factors) = (floor_sqrt(value), 0);
    while f > 0 {
        if if value <= i32::max_value() as _ { value as i32 % f as i32 == 0 } else { value % f == 0 } {
            factors += 1;
        }
        f -= 1;
    }
    // need to adjust the number of divisors if the number is a perfect square
    (factors << 1) - if is_square(value) { 1 } else { 0 }
}

// defined according to problem 21: numbers less than n which divide evenly into n
pub fn sum_of_factors(value: isize) -> isize {
    let (mut f, mut sum) = (floor_sqrt(value), 1);
    while f > 1 {
        if if value <= i32::max_value() as _ { value as i32 % f as i32 == 0 } else { value % f == 0 } {
            sum += f + value / f;
        }
        f -= 1;
    }
    // need to adjust the number of divisors if the number is a perfect square
    if is_square(value) { sum - floor_sqrt(value) } else { sum }
}

pub fn is_abundant(value: isize) -> bool {
    value < sum_of_factors(value)
}