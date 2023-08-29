// COPYRIGHT (C) 2022 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::cast::Cast;
use algorithm::digits::{DEFAULT_RADIX, Digits, digits_iter, nth_digit};
use algorithm::factor::sum_of_factors;
use algorithm::prime::miller_rabin;
use algorithm::root::int_log_10;

/// verifies if a given value is even
#[must_use]
pub fn is_even(value: &i64) -> bool {
    value & 1 == 0
}

/// verifies if a given value is even
#[must_use]
pub fn is_even_u64(value: &u64) -> bool {
    value & 1 == 0
}

/// verifies if a given value is odd
#[must_use]
pub fn is_odd(value: &i64) -> bool {
    value & 1 != 0
}

/// verifies if a given value is odd
#[must_use]
pub fn is_odd_u64(value: &u64) -> bool {
    value & 1 != 0
}

/// verifies if a given value is positive
#[must_use]
pub fn is_positive(&value: &i64) -> bool {
    value >= 0
}

// --- //

/// verifies if a given array is of a certain size
#[must_use]
#[allow(clippy::type_complexity)]
pub fn is_len<T>(size: usize) -> Box<dyn FnMut(&[T]) -> bool> {
    Box::new(move |value| value.len() == size)
}

// --- //

/// creates a closure to filter values equal to
#[must_use]
pub fn equals(other: i64) -> Box<dyn FnMut(&i64) -> bool> {
    Box::new(move |&value| value == other)
}

/// creates a closure to filter values equal to
#[must_use]
pub fn equals_u64(other: u64) -> Box<dyn FnMut(&u64) -> bool> {
    Box::new(move |&value| value == other)
}

/// creates a closure to filter values lesser than
#[must_use]
pub fn less_than(other: i64) -> Box<dyn FnMut(&i64) -> bool> {
    Box::new(move |&value| value < other)
}

/// creates a closure to filter values lesser than
#[must_use]
pub fn less_than_u64(other: u64) -> Box<dyn FnMut(&u64) -> bool> {
    Box::new(move |&value| value < other)
}

/// creates a closure to filter values lesser than
#[must_use]
pub fn less_than_usize(other: usize) -> Box<dyn FnMut(&usize) -> bool> {
    Box::new(move |&value| value < other)
}

/// creates a closure to filter values lesser or equal than
#[must_use]
pub fn less_or_equal_than(other: i64) -> Box<dyn FnMut(&i64) -> bool> {
    Box::new(move |&value| value <= other)
}

/// creates a closure to filter values lesser or equal than
#[must_use]
pub fn less_or_equal_than_u64(other: u64) -> Box<dyn FnMut(&u64) -> bool> {
    Box::new(move |&value| value <= other)
}

/// creates a closure to filter values greater than
#[must_use]
pub fn greater_than(other: i64) -> Box<dyn FnMut(&i64) -> bool> {
    Box::new(move |&value| value > other)
}

/// creates a closure to filter values greater or equal than
#[must_use]
pub fn greater_or_equal_than(other: i64) -> Box<dyn FnMut(&i64) -> bool> {
    Box::new(move |&value| value >= other)
}

// --- //

/// quick test to find out if an arbitrary value is prime
#[must_use]
pub fn is_prime(&value: &u64) -> bool {
    value == 2 || value & 1 != 0 && miller_rabin(value)
}

/// if the sum of the factors of a given value equals the value itself
#[must_use]
pub fn is_abundant(&value: &i64) -> bool {
    value < sum_of_factors(value)
}

/// if the sum of the factors of a given value equals the value itself
#[must_use]
pub fn is_abundant_usize(&value: &usize) -> bool {
    value.as_i64() < sum_of_factors(value.as_i64())
}

// --- //

/// checks if a value has repeated digits
#[must_use]
pub fn is_unique_digits(&value: &u64) -> bool {
    Digits::from(value).is_unique()
}

/// verifies if a given value is a palindrome, i.e. reads the same both ways
#[must_use]
pub const fn is_palindrome(&value: &u64) -> bool {
    // changed to const: (0..size / 2).all(|i| nth_digit(value, i) == nth_digit(value, size - i - 1))
    let size = int_log_10(value);
    if size <= 1 {
        return true;
    }
    let mut i = 0;
    loop {
        if nth_digit(value, i) != nth_digit(value, size - i - 1) {
            return false;
        }
        i += 1;
        if i == size / 2 {
            return true;
        }
    }
}

/// verifies if a given number is pandigital, i.e. it utilizes all the numbers
#[must_use]
pub fn is_pandigital(&value: &u64) -> bool {
    let mut seen = vec![false; DEFAULT_RADIX as usize];
    seen[0] = true;

    digits_iter(value).map(|d| d.as_usize()).try_for_each(|d| (!seen[d]).then(|| seen[d] = true));
    seen.iter().take(1 + int_log_10(value).as_usize()).all(|&seen| seen)
}
