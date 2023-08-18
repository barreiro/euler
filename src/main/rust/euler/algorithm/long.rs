// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::digits::Digit;
use algorithm::root::{int_sqrt, is_square};

pub trait IncrementAndGet {
    #[allow(clippy::return_self_not_must_use)]
    fn increment_and_get(&mut self) -> Self;
}

impl IncrementAndGet for i32 {
    fn increment_and_get(&mut self) -> Self {
        *self += 1;
        *self
    }
}
impl IncrementAndGet for Digit {
    fn increment_and_get(&mut self) -> Self {
        *self += 1;
        *self
    }
}

impl IncrementAndGet for u64 {
    fn increment_and_get(&mut self) -> Self {
        *self += 1;
        *self
    }
}

impl IncrementAndGet for usize {
    fn increment_and_get(&mut self) -> Self {
        *self += 1;
        *self
    }
}

pub trait GetAndIncrement {
    #[allow(clippy::return_self_not_must_use)]
    fn get_and_increment(&mut self) -> Self;
}

impl GetAndIncrement for i32 {
    fn get_and_increment(&mut self) -> Self {
        let value = *self;
        *self += 1;
        value
    }
}

impl GetAndIncrement for i64 {
    fn get_and_increment(&mut self) -> Self {
        let value = *self;
        *self += 1;
        value
    }
}

impl GetAndIncrement for u64 {
    fn get_and_increment(&mut self) -> Self {
        let value = *self;
        *self += 1;
        value
    }
}

impl GetAndIncrement for usize {
    fn get_and_increment(&mut self) -> Self {
        let value = *self;
        *self += 1;
        value
    }
}

// --- //

/// calculates the Greatest Common Divisor using Stein's algorithm
#[allow(clippy::manual_swap)] // because of const
#[must_use]
pub const fn gcd(mut a: i64, mut b: i64) -> i64 {
    if a == 0 { return b; }
    if b == 0 { return a; }
    let shift = (a | b).trailing_zeros();
    a >>= a.trailing_zeros();
    while b != 0 {
        b >>= b.trailing_zeros();
        if a > b {
            // swap(&mut a, &mut b);
            let tmp = a;
            a = b;
            b = tmp;
        }
        b -= a;
    }
    a << shift
}

/// checks that two values are coprime, i.e. don't have common factors except for 1
#[must_use]
pub const fn are_coprime(a: i64, b: i64) -> bool {
    gcd(a, b) == 1
}

/// calculates the Least Common Multiple
#[must_use]
pub const fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}

/// calculates quotient and remainder in one go
#[must_use]
pub const fn div_rem(a: i64, b: i64) -> (i64, i64) {
    (a / b, a % b)
}

/// calculates a quotient that leaves negative remainder
#[must_use]
pub const fn div_ceil(a: i64, b: i64) -> i64 {
    let (q, r) = div_rem(a, b);
    if r == 0 { q } else { q + 1 }
}

/// the sum of all the numbers up to value
#[must_use]
pub const fn arithmetic_sum(value: i64) -> i64 {
    (value * (value + 1)) / 2
}

/// the sum of all the numbers up to value
#[must_use]
pub const fn arithmetic_sum_u64(value: u64) -> u64 {
    (value * (value + 1)) / 2
}

/// simple method to calculate the factorial of small values. No checks are performed. Use with caution.
#[must_use]
pub fn factorial(value: u64) -> u64 {
    (2..=value).product()
}

// --- //

#[must_use]
pub const fn triangle(value: i64) -> i64 {
    arithmetic_sum(value)
}

#[must_use]
pub const fn is_triangle(&value: &i64) -> bool {
    is_square(value * 8 + 1)
}

#[must_use]
pub const fn pentagonal(value: i64) -> i64 {
    (value * (3 * value - 1)) / 2
}

#[must_use]
pub const fn is_pentagonal(&value: &i64) -> bool {
    value == pentagonal(int_sqrt(((value + 1) * 2) / 3))
}

#[must_use]
pub const fn hexagonal(value: i64) -> i64 {
    value * (value * 2 - 1)
}

#[must_use]
pub const fn is_hexagonal(&value: &i64) -> bool {
    value == hexagonal(int_sqrt((value + 1) / 2))
}

#[must_use]
pub const fn heptagonal(value: i64) -> i64 {
    value * (5 * value - 3) / 2
}

#[must_use]
pub const fn octagonal(value: i64) -> i64 {
    value * (3 * value - 2)
}

// --- //

/// efficiently computes `(base ^ exp) mod modulo`
#[must_use]
pub const fn pow_mod(base: u64, exp: u64, modulo: u64) -> u64 {
    let (mut result, mut b, mut e) = (1, base % modulo, exp);
    while e > 0 {
        if e & 1 != 0 {
            result = mul_mod(result, b, modulo);
        }
        e >>= 1;
        b = mul_mod(b, b, modulo);
    }
    result
}

/// computes `(a * b) mod modulo` without overflowing the multiplication
#[must_use]
pub const fn mul_mod(a: u64, mut b: u64, modulo: u64) -> u64 {
    if let Some(result) = a.checked_mul(b) {
        result % modulo
    } else {
        let (mut result, mut j) = (0, a % modulo);
        while b != 0 {
            if b & 1 != 0 {
                result = (result + j) % modulo;
            }
            j = (j * 2) % modulo;
            b >>= 1;
        }
        result % modulo
    }
}
