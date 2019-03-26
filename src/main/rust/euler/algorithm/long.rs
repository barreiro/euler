// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::alloc::{alloc, Layout};
use std::mem;

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
pub fn pow_10(exp: isize) -> isize {
    POW_10[exp as usize]
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
    if approx > result {
        result as isize + 1
    } else {
        result as isize
    }
}

/// the sum of all the numbers up to value
pub fn arithmetic_sum(value: isize) -> isize {
    value * (value + 1) / 2
}

pub fn factorial(value: isize) -> isize {
    let mut prod = 1;
    for l in 1..value + 1 {
        prod *= l
    }
    prod
}

pub fn pow(base: isize, exp: isize) -> isize {
    if base == 0 && exp == 0 {
        return 1;
    }
    if base == 0 {
        return 0;
    }
    if base == 1 {
        return base;
    }
    if base == 2 {
        return 1 << exp;
    }
    if base == 10 {
        return pow_10(exp);
    }

    if exp == 0 {
        return 1;
    }
    if exp == 1 {
        return base;
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

pub fn square(base: isize) -> isize {
    base * base
}

// --- //

pub fn is_palindrome(value: isize) -> bool {
    is_palindrome_digits(to_digits(value))
}

fn is_palindrome_digits(digits: Vec<isize>) -> bool {
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

// --- //

pub fn to_digits(value: isize) -> Vec<isize> {
    to_digits_radix(value, DEFAULT_RADIX)
}

fn to_digits_radix(mut value: isize, radix: isize) -> Vec<isize> {
    // fast write of digits
    let (mut len, size) = (0, 32 / int_sqrt(radix) as usize);
    let digits_ptr = unsafe { alloc(Layout::from_size_align_unchecked(mem::size_of::<isize>() * size, mem::align_of::<isize>())) as *mut isize };

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
