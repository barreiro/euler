// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::alloc::{alloc, Layout};
use std::mem::{align_of, size_of};

pub const fn is_even(&l: &isize) -> bool {
    l % 2 == 0
}

pub const fn is_odd(&l: &isize) -> bool {
    l % 2 != 0
}

// --- //

// Table for fast lookup of powers of 10
const POW_10: [isize; 19] = [
    1,
    10,
    100,
    1000,
    10000,
    100000,
    1000000,
    10000000,
    100000000,
    1000000000,
    10000000000,
    100000000000,
    1000000000000,
    10000000000000,
    100000000000000,
    1000000000000000,
    10000000000000000,
    100000000000000000,
    1000000000000000000,
];

pub const DEFAULT_RADIX: isize = 10;

/// Convenience method to calculate the power when in base 10.
pub const fn pow_10(exp: isize) -> isize {
    POW_10[exp as usize]
}

/// Convenience method to calculate the integer logarithm in base 10.
pub fn int_log_10(n: isize) -> isize {
    POW_10.iter().take_while(|&&d| d <= n).count() as isize
}

/// calculates an approximate of the square root
pub fn int_sqrt(value: isize) -> isize {
    // fast path for digit conversions on the default radix
    if value == DEFAULT_RADIX {
        return 3;
    }

    let (mut approx, mut one, mut result) = (value, 1 << 30, 0);

    // "one" starts at the highest power of four <= than the argument
    while one > value {
        one >>= 2
    }

    while one != 0 {
        if approx >= result + one {
            approx -= result + one;
            result = result + (one << 1);
        }
        one >>= 2;
        result >>= 1;
    }

    // Rounding to nearest integer
    result + if approx > result { 1 } else { 0 }
}

/// the sum of all the numbers up to value
pub const fn arithmetic_sum(value: isize) -> isize {
    value * (value + 1) / 2
}

/// Simple method to calculate the factorial of small values. No checks are performed. Use with caution.
pub fn factorial(value: isize) -> isize {
    let mut prod = 1;
    for l in 1..=value {
        prod *= l
    }
    prod
}

pub fn pow(base: isize, exp: isize) -> isize {
    if base == 10 {
        return POW_10[exp as usize];
    }

    if exp == 0 {
        return 1;
    }
    if base == 0 {
        return 0;
    }
    if base == 1 || exp == 1 {
        return base;
    }
    if base == 2 {
        return 1 << exp;
    }
    if exp == 2 {
        return base * base;
    }
    squaring(base, exp)
}

fn squaring(base: isize, exp: isize) -> isize {
    let (mut sqr_base, mut sqr_exp, mut result) = (base, exp, 1);
    loop {
        if sqr_exp == 0 {
            return result;
        }
        if sqr_exp % 2 != 0 {
            result *= sqr_base;
        }
        sqr_base = square(sqr_base);
        sqr_exp = sqr_exp / 2;
    }
}

pub const fn square(base: isize) -> isize {
    base * base
}

pub fn is_perfect(value: isize) -> bool {
    square(int_sqrt(value)) == value
}

// --- //

// triangle == arithmetic_sum

pub fn is_triangle(&value: &isize) -> bool {
    value == arithmetic_sum(int_sqrt(2 * value))
}

pub const fn pentagonal(value: isize) -> isize {
    value * (3 * value - 1) / 2
}

pub fn is_pentagonal(&value: &isize) -> bool {
    value == pentagonal(int_sqrt(2 * (value + 1) / 3))
}

pub const fn hexagonal(value: isize) -> isize {
    value * (2 * value - 1)
}

// --- //

pub fn is_palindrome(value: isize) -> bool {
    is_palindrome_digits(&to_digits(value))
}

pub fn is_palindrome_radix(value: isize, radix: isize) -> bool {
    is_palindrome_digits(&to_digits_radix(value, radix))
}

fn is_palindrome_digits(digits: &[isize]) -> bool {
    let (mut l, len, digits_ptr) = (digits.len() as isize / 2, digits.len() as isize, digits.as_ptr());
    while l != 0 {
        // fast read of digits
        if unsafe { *digits_ptr.offset(l) != *digits_ptr.offset(len - l - 1) } {
            return false;
        }
        l -= 1;
    }
    unsafe { *digits_ptr == *digits_ptr.offset(len - 1) }
}

/// Tests if a given number is pandigital, i.e. it has all the digits one and only once (excluding zero)
pub fn is_pandigital(digits: &[isize]) -> bool {
    for i in 0..digits.len() as isize {
        if !digits.contains(&(i + 1)) {
            return false;
        }
    }
    return true;
}

/// Concatenates two digits into a new one. First argument becomes more significant and second argument becomes less significant.
pub fn concatenation(digit1: &[isize], digit2: &[isize]) -> Vec<isize> {
    let mut array = Vec::with_capacity(digit1.len() + digit2.len());
    array.extend_from_slice(digit2);
    array.extend_from_slice(digit1);
    array
}

// --- //

pub fn last_digits(value: isize, n: isize) -> isize {
    value % POW_10[n as usize]
}

pub fn first_digits(value: isize, n: isize) -> isize {
    value / POW_10[(int_log_10(value).max(n) - n) as usize]
}

pub fn nth_digit(value: isize, n: isize) -> isize {
    value / POW_10[(int_log_10(value) - n) as usize] % DEFAULT_RADIX
}

pub fn to_digits(value: isize) -> Vec<isize> {
    to_digits_radix(value, DEFAULT_RADIX)
}

fn to_digits_radix(mut value: isize, radix: isize) -> Vec<isize> {
    // fast write of digits
    let (mut len, size) = (0, 32 / int_sqrt(radix) as usize);
    let digits_ptr = unsafe { alloc(Layout::from_size_align_unchecked(size_of::<isize>() * size, align_of::<isize>())) as *mut isize };

    while value >= radix {
        unsafe { *digits_ptr.offset(len) = value % radix };
        len += 1;

        value /= radix;
    }
    unsafe {
        *digits_ptr.offset(len) = value;
        Vec::from_raw_parts(digits_ptr, len as usize + 1, size)
    }
}

// --- //

// this consumes the digits. others only borrow them.
pub fn from_digits(digits: Vec<isize>) -> isize {
    from_digits_index_radix(&digits, 0, digits.len(), DEFAULT_RADIX)
}

pub fn from_digits_index(digits: &[isize], from: usize, to: usize) -> isize {
    from_digits_index_radix(digits, digits.len() - to, digits.len() - from, DEFAULT_RADIX)
}

fn from_digits_index_radix(digits: &[isize], from: usize, to: usize, radix: isize) -> isize {
    let (mut result, mut i, base_10) = (0, from, radix == DEFAULT_RADIX);
    while i < to {
        result += digits[i] * if base_10 { POW_10[i - from] } else { pow(radix, (i - from) as isize) };
        i += 1;
    }
    result
}

// --- //

pub struct Decrementing {
    value: isize
}

pub fn decrementing(initial: isize) -> Decrementing {
    Decrementing { value: initial }
}

impl Iterator for Decrementing {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        self.value -= 1;
        if self.value > 0 {
            Some(self.value)
        } else {
            None
        }
    }
}

// --- //

pub struct IncrementingDigits {
    array: Vec<isize>
}

pub fn incrementing_digits(initial: isize) -> IncrementingDigits {
    IncrementingDigits { array: to_digits(initial) }
}

impl Iterator for IncrementingDigits {
    type Item = Vec<isize>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.increase() && !self.rotate() {
            self.expand();
        }
        Some(self.array.to_vec())
    }
}

impl IncrementingDigits {
    fn increase(&mut self) -> bool {
        if self.array[0] < 9 {
            self.array[0] = self.array[0] + 1;
            return true;
        }
        return false;
    }

    fn rotate(&mut self) -> bool {
        for i in 1..self.array.len() {
            self.array[i - 1] = 0;
            if self.array[i] != 9 {
                self.array[i] = self.array[i] + 1;
                return true;
            }
        }
        return false;
    }

    fn expand(&mut self) {
        let size = self.array.len();
        self.array.clear();
        self.array.resize(size + 1, 0);
        self.array[size] = 1;
    }
}

// --- //

pub fn power_modulo(base: isize, exp: isize, modulo: isize) -> isize {
    let (mut result, mut b, mut e) = (1, base % modulo, exp);
    if (modulo - 1).checked_mul(modulo - 1).is_none() {
        return 0;
    }

    while e > 0 {
        if e & 1 != 0 {
            result = result * b % modulo;
        }
        e = e >> 1;
        b = b * b % modulo;
    }
    if result < 0 { result + modulo } else { result }
}
