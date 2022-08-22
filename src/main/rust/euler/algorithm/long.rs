// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

pub const fn is_even(l: isize) -> bool {
    l & 1 == 0
}

pub const fn is_odd(l: isize) -> bool {
    l & 1 == 1
}

// --- //

/// constant that defines the default base for conversions to and from digits
pub const DEFAULT_RADIX: isize = 10;

// table for fast lookup of powers of 10
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

/// convenience method to calculate the power when in base 10.
pub const fn pow_10(exp: isize) -> isize {
    POW_10[exp as usize]
}

/// convenience method to calculate the integer logarithm in base 10.
pub const fn int_log_10(n: isize) -> isize {
    // POW_10.iter().take_while(|&&d| d <= n).count() as _
    let mut i = 0;
    while POW_10[i] <= n {
        i += 1;
    }
    i as _
}

/// calculates the Greatest Common Divisor using Stein's algorithm
#[allow(clippy::manual_swap)] // because of const
pub const fn gcd(mut a: isize, mut b: isize) -> isize {
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

/// calculates the Least Common Multiple
pub const fn lcm(a: isize, b: isize) -> isize {
    a / gcd(a, b) * b
}

/// calculates quotient and remainder in one go
pub const fn div_rem(a: isize, b: isize) -> (isize, isize) {
    (a / b, a % b)
}

/// calculates a quotient that leaves negative remainder
pub const fn div_ceil(a: isize, b: isize) -> isize {
    let (q, r) = div_rem(a, b);
    if r == 0 { q } else { q + 1 }
}

/// calculates an approximation of the square root
pub const fn floor_sqrt(value: isize) -> isize {
    exact_sqrt(value).0
}

/// calculates an approximation of the square root
pub const fn floor_sqrt_usize(value: usize) -> usize {
    exact_sqrt(value as _).0 as _
}

/// calculates an overestimation of the square root
pub const fn ceil_sqrt(value: isize) -> isize {
    let (root, remainder) = exact_sqrt(value);
    if remainder == 0 { root } else { root + 1 }
}

/// calculates an approximation of the square root
pub const fn int_sqrt(value: isize) -> isize {
    let (root, remainder) = exact_sqrt(value);
    // rounding to nearest integer
    if remainder > root { root + 1 } else { root }
}

pub const fn exact_sqrt(value: isize) -> (isize, isize) {
    // fast path for digit conversions on the default radix
    if value == DEFAULT_RADIX {
        return (3, 1);
    } else if value == 0 {
        return (0, 0);
    }

    // "place" starts at the highest power of four <= than the argument (64 = 0.leading_zeros())
    let leading = 64 - value.leading_zeros();
    let (mut remainder, mut place, mut root) = (value, 1 << if leading & 1 == 0 { leading - 2 } else { leading - 1 }, 0);

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
    (root, remainder)
}

/// calculates an approximation of the cube root
pub const fn floor_cube_root(value: isize) -> isize {
    exact_root(value, 3).0
}

/// returns nth root of a number value (binary search algorithm)
pub const fn exact_root(value: isize, n: isize) -> (isize, isize) {
    // set start and end for binary search as some powers of 2
    let leading = (63 - value.leading_zeros() as isize) / n;
    let (mut start, mut end) = (1 << leading, 1 << (leading + 1));
    loop {
        if end - start == 1 { break (start, value - pow(start, n)); }

        let mid = (start + end) >> 1;
        let error = value - pow(mid, n);
        if error.is_negative() { end = mid } else { start = mid }
    }
}

/// the sum of all the numbers up to value
pub const fn arithmetic_sum(value: isize) -> isize {
    (value * (value + 1)) >> 1
}

/// simple method to calculate the factorial of small values. No checks are performed. Use with caution.
pub fn factorial(value: isize) -> isize {
    (2..=value).product()
}

/// power!
pub const fn pow(base: isize, exp: isize) -> isize {
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
    if base == -1 {
        return if is_even(exp) { 1 } else { -1 };
    }
    if base == 2 {
        return 1 << exp;
    }
    if exp == 2 {
        return base * base;
    }
    squaring(base, exp)
}

const fn squaring(base: isize, exp: isize) -> isize {
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

/// calculates the square af a given value
pub const fn square(base: isize) -> isize {
    base * base
}

/// verifies that a given value is a perfect square
pub const fn is_square(value: isize) -> bool {
    let hex = value & 0xF; // last hexadecimal "digit" (has to be either 0, 1, 4 or 9)
    hex <= 9 && (hex == 0 || hex == 1 || hex == 4 || hex == 9) && exact_sqrt(value).1 == 0
}

/// calculates the cube of a given value
pub const fn cube(base: isize) -> isize {
    base * base * base
}

/// verifies that a given value is a perfect cube
pub const fn is_cube(base: isize) -> bool {
    exact_root(base, 3).1 == 0
}

/// calculates the fourth power of a given value
pub const fn fourth(base: isize) -> isize {
    square(base) * square(base)
}

// --- //

pub const fn triangle(value: isize) -> isize {
    arithmetic_sum(value)
}

pub const fn is_triangle(value: isize) -> bool {
    is_square((value << 3) + 1)
}

pub const fn pentagonal(value: isize) -> isize {
    (value * (3 * value - 1)) >> 1
}

pub const fn is_pentagonal(value: isize) -> bool {
    value == pentagonal(int_sqrt(((value + 1) << 1) / 3))
}

pub const fn hexagonal(value: isize) -> isize {
    value * ((value << 1) - 1)
}

pub const fn is_hexagonal(value: isize) -> bool {
    value == hexagonal(int_sqrt((value + 1) >> 1))
}

pub const fn heptagonal(value: isize) -> isize {
    (value * (5 * value - 3)) >> 1
}

pub const fn octagonal(value: isize) -> isize {
    value * (3 * value - 2)
}

// --- //

/// verifies if a given value is a palindrome, i.e. reads the same both ways
pub const fn is_palindrome(value: isize) -> bool {
    // changed to const: (0..size / 2).all(|i| nth_digit(value, i) == nth_digit(value, size - i - 1))
    let size = int_log_10(value);
    let mut i = (size >> 1) - 1;
    while i >= 0 {
        if nth_digit(value, i) != nth_digit(value, size - i - 1) {
            return false;
        }
        i -= 1;
    }
    true
}

/// verifies if a given value is a palindrome, i.e. reads the same both ways is a given base
pub fn is_palindrome_radix(value: isize, radix: isize) -> bool {
    is_palindrome_digits(&to_digits_radix(value, radix))
}

/// verifies that an array of digits is a palindrome, i.e. reads the same both ways
pub const fn is_palindrome_digits(digits: &[isize]) -> bool {
    // changed to const: (0..=digits.len() / 2).all(|i| digits[i] == digits[digits.len() - i - 1])
    let mut i = digits.len() >> 1;
    loop {
        if digits[i] != digits[digits.len() - i - 1] {
            return false;
        }
        if i == 0 {
            return true;
        }
        i -= 1;
    }
}

/// tests if a given number is pandigital, i.e. it has all the digits one and only once (excluding zero)
pub fn is_pandigital(digits: &[isize]) -> bool {
    for i in 0..digits.len() as _ {
        if !digits.contains(&(i + 1)) {
            return false;
        }
    }
    true
}

/// concatenate two values
pub const fn concatenation(one: isize, two: isize) -> isize {
    one * pow_10(int_log_10(two)) + two
}

// --- //

/// checks if a value has repeated digits
pub fn unique_digits(value: isize) -> bool {
    let mut digits = to_digits(value);
    digits.sort_unstable();
    (1..digits.len()).all(|i| digits[i] != digits[i - 1])
}

/// checks if two values are permutations of one another, i.e. have the same digits but it different order
pub fn is_permutation(a: isize, b: isize) -> bool {
    if a % 9 == b % 9 {
        let (mut digits_a, mut digits_b) = (to_digits(a), to_digits(b));
        if digits_a.len() == digits_b.len() {
            digits_a.sort_unstable();
            digits_b.sort_unstable();
            return (0..digits_a.len()).all(|i| digits_a[i] == digits_b[i]);
        }
    }
    false
}

/// calculates the sum of the digits of a given value
pub const fn digits_sum(value: &isize) -> isize {
    let (mut sum, mut v) = (0, *value);
    while v >= DEFAULT_RADIX {
        sum += v % DEFAULT_RADIX;
        v /= DEFAULT_RADIX;
    }
    sum + v
}

pub const fn digits_square_sum(mut value: isize) -> isize {
    let mut sum = 0;
    while value >= DEFAULT_RADIX {
        let remainder = value % DEFAULT_RADIX;
        sum += remainder * remainder;
        value /= DEFAULT_RADIX;
    }
    sum + value * value
}

pub const fn last_digits(value: isize, n: isize) -> isize {
    value % POW_10[n as usize]
}

pub const fn first_digits(value: isize, n: isize) -> isize {
    let int_log = int_log_10(value); // alternative to max() function for const
    value / POW_10[((if int_log > n { int_log } else { n }) - n) as usize]
}

pub const fn nth_digit(value: isize, n: isize) -> isize {
    value / POW_10[(int_log_10(value) - n - 1) as usize] % DEFAULT_RADIX
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

pub const fn from_digits_array(digits: &[isize]) -> isize {
    from_digits_index_radix(digits, 0, digits.len(), DEFAULT_RADIX)
}

pub const fn from_digits_index(digits: &[isize], from: usize, to: usize) -> isize {
    from_digits_index_radix(digits, digits.len() - to, digits.len() - from, DEFAULT_RADIX)
}

const fn from_digits_index_radix(digits: &[isize], from: usize, to: usize, radix: isize) -> isize {
    let (mut result, mut i, base_10) = (0, from, radix == DEFAULT_RADIX);
    while i < to {
        result += digits[i] * if base_10 { POW_10[i - from] } else { pow(radix, (i - from) as _) };
        i += 1;
    }
    result
}

// --- //

/// provides an iterator of numbers in digit format, starting at an initial value
pub fn incrementing_digits(initial: isize) -> impl Iterator<Item=Vec<isize>> {
    IncrementingDigits { array: to_digits(initial - 1) }
}

struct IncrementingDigits {
    array: Vec<isize>,
}

impl Iterator for IncrementingDigits {
    type Item = Vec<isize>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.increase() && !self.rotate() {
            self.expand();
        }
        Some(self.array.clone())
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

/// efficiently computes `(base ^ exp) mod modulo`
pub const fn power_modulo(base: isize, exp: isize, modulo: isize) -> isize {
    let (mut result, mut b, mut e) = (1, base % modulo, exp);
    while e > 0 {
        if e & 1 != 0 {
            result = safe_mul_mod(result, b, modulo);
        }
        e >>= 1;
        b = safe_mul_mod(b, b, modulo);
    }
    result
}

/// computes `(a * b) mod modulo` without overflowing the multiplication
pub const fn safe_mul_mod(a: isize, mut b: isize, modulo: isize) -> isize {
    if let Some(result) = a.checked_mul(b) {
        result % modulo
    } else {
        let (mut i, mut j) = (0, a);
        while b != 0 {
            if b & 1 != 0 {
                i = (i + j) % modulo;
            }
            j = (j << 1) % modulo;
            b >>= 1;
        }
        i % modulo
    }
}
