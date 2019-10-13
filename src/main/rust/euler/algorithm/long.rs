// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

pub const fn is_even(l: isize) -> bool {
    l & 1 == 0
}

pub const fn is_odd(l: isize) -> bool {
    l & 1 == 1
}

// --- //

// Table for fast lookup of powers of 4
const POW_4: [isize; 32] = [
    0x0001,
    0x0004,
    0x0010,
    0x0040,
    0x0100,
    0x0400,
    0x1000,
    0x4000,
    0x0001_0000,
    0x0004_0000,
    0x0010_0000,
    0x0040_0000,
    0x0100_0000,
    0x0400_0000,
    0x1000_0000,
    0x4000_0000,
    0x0001_0000_0000,
    0x0004_0000_0000,
    0x0010_0000_0000,
    0x0040_0000_0000,
    0x0100_0000_0000,
    0x0400_0000_0000,
    0x1000_0000_0000,
    0x4000_0000_0000,
    0x0001_0000_0000_0000,
    0x0004_0000_0000_0000,
    0x0010_0000_0000_0000,
    0x0040_0000_0000_0000,
    0x0100_0000_0000_0000,
    0x0400_0000_0000_0000,
    0x1000_0000_0000_0000,
    0x4000_0000_0000_0000,
];

// Table for fast lookup of powers of 10
const POW_10: [isize; 19] = [
    1,
    10,
    100,
    1_000,
    10_000,
    100_000,
    1_000_000,
    10_000_000,
    100_000_000,
    1_000_000_000,
    10_000_000_000,
    100_000_000_000,
    1_000_000_000_000,
    10_000_000_000_000,
    100_000_000_000_000,
    1_000_000_000_000_000,
    10_000_000_000_000_000,
    100_000_000_000_000_000,
    1_000_000_000_000_000_000,
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

    // "place" starts at the highest power of four <= than the argument
    let (mut remainder, mut place, mut root) = (value, *POW_4.iter().take_while(|&&d| d <= value).last().unwrap(), 0);

    while place != 0 {
        let term = root + place;
        if remainder >= term {
            remainder -= term;
            root = (root >> 1) + place;
        } else {
            root >>= 1;
        }
        place >>= 2;
    }

    // Rounding to nearest integer
    if remainder > root { root + 1 } else { root }
}

/// the sum of all the numbers up to value
pub const fn arithmetic_sum(value: isize) -> isize {
    value * (value + 1) / 2
}

/// Simple method to calculate the factorial of small values. No checks are performed. Use with caution.
pub fn factorial(value: isize) -> isize {
    (2..=value).product()
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
        if sqr_exp & 1 != 0 {
            result *= sqr_base;
        }
        sqr_base *= sqr_base;
        sqr_exp /= 2;
    }
}

pub const fn square(base: isize) -> isize {
    base * base
}

pub fn is_perfect_square(value: isize) -> bool {
    square(int_sqrt(value)) == value
}

// --- //

// triangle == arithmetic_sum

pub fn is_triangle(value: isize) -> bool {
    value == arithmetic_sum(int_sqrt(2 * value))
}

pub const fn pentagonal(value: isize) -> isize {
    value * (3 * value - 1) / 2
}

pub fn is_pentagonal(value: isize) -> bool {
    value == pentagonal(int_sqrt(2 * (value + 1) / 3))
}

pub const fn hexagonal(value: isize) -> isize {
    value * (2 * value - 1)
}

// --- //

pub fn is_palindrome(value: isize) -> bool {
    let (size, nth) = (int_log_10(value), |n| value / POW_10[n as usize] % DEFAULT_RADIX);
    (0..size / 2).all(|i| nth(i) == nth(size - i - 1))
}

pub fn is_palindrome_radix(value: isize, radix: isize) -> bool {
    is_palindrome_digits(&to_digits_radix(value, radix))
}

pub fn is_palindrome_digits(digits: &[isize]) -> bool {
    (0..=digits.len() / 2).all(|i| digits[i] == digits[digits.len() - i - 1])
}

/// Tests if a given number is pandigital, i.e. it has all the digits one and only once (excluding zero)
pub fn is_pandigital(digits: &[isize]) -> bool {
    for i in 0..digits.len() as isize {
        if !digits.contains(&(i + 1)) {
            return false;
        }
    }
    true
}

pub fn concatenation(one: isize, two: isize) -> isize {
    one * pow_10(int_log_10(two)) + two
}

// --- //

pub fn digits_sum(mut value: isize) -> isize {
    let mut sum = 0;
    while value >= DEFAULT_RADIX {
        sum += value % DEFAULT_RADIX;
        value /= DEFAULT_RADIX;
    }
    sum + value
}

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
    let mut digits = Vec::with_capacity(12);
    while value >= radix {
        digits.push(value % radix);
        value /= radix;
    }
    digits.push(value);
    digits
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

pub struct IncrementingDigits {
    array: Vec<isize>
}

pub fn incrementing_digits(initial: isize) -> IncrementingDigits {
    IncrementingDigits { array: to_digits(initial - 1) }
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
            self.array[0] += 1;
            return true;
        }
        false
    }

    fn rotate(&mut self) -> bool {
        for i in 1..self.array.len() {
            self.array[i - 1] = 0;
            if self.array[i] != 9 {
                self.array[i] += 1;
                return true;
            }
        }
        false
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
            result *= b;
            result %= modulo;
        }
        e >>= 1;
        b *= b;
        b %= modulo;
    }
    if result < 0 { result + modulo } else { result }
}
