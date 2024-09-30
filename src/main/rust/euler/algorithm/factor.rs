// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::convert::TryFrom;

use crate::algorithm::cast::Cast;
use crate::algorithm::root::{floor_sqrt, square};

/// test if a given value has a factor pair where both factors are below a given bound
#[must_use]
pub const fn has_factor_pair_below(value: i64, bound: i64) -> bool {
    // changed for const: (floor_sqrt(value)..bound).any(|l| value % l == 0 && value / l < bound)
    let mut i = floor_sqrt(value);
    while i < bound {
        if value % i == 0 && value / i < bound {
            return true;
        }
        i += 1;
    }
    false
}

/// Euler's totient function counts the positive integers up to a given integer n that are relatively prime to n
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub const fn totient(value: u64) -> u64 {
    let (mut f, mut n, mut totient) = (2, value, value);
    while f * f <= n {
        if if n <= u32::MAX as u64 { n as u32 % f as u32 == 0 } else { n % f == 0 } {
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

/// the number of different divisors of a given value
#[must_use]
pub fn number_of_factors(value: i64) -> u64 {
    // need to adjust the number of divisors if the number is a perfect square
    // (proper_factors_of(value).count().as_u64() << 1) - u64::from(is_square(value))
    let mut first_factor = 0;
    (proper_factors_of(value).inspect(|&f| if first_factor == 0 { first_factor = f }).count() << 1).as_u64() - u64::from(square(first_factor) == value)

    // alternative implementation with prime factorization
    // prime_factors(value.as_u64()).values().map(|f| f + 1).product()
}

/// calculate the sum of the factors of a given value
// defined according to problem 21: numbers less than n which divide evenly into n
#[must_use]
pub fn sum_of_factors(value: i64) -> i64 {
    let mut first_factor = 0;
    let sum = proper_factors_of(value).inspect(|&f| if first_factor == 0 { first_factor = f }).map(|f| f + value / f).sum::<i64>();

    // need to adjust the sum if the number is a perfect square
    if square(first_factor) == value { sum - value - first_factor } else { sum - value }
}

// --- //

/// provides an iterator of the proper factors of value, the factors up until the square root
#[allow(clippy::cast_possible_truncation)]
pub fn proper_factors_of(value: i64) -> impl Iterator<Item=i64> {
    let small = i32::try_from(value).is_ok();
    (1..=floor_sqrt(value)).rev().filter(move |&f| if small { value as i32 % f as i32 == 0 } else { value % f == 0 })
}
