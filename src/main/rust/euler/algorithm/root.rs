// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::digits::DEFAULT_RADIX;

// table for fast lookup of powers of 10
const POW_10: [u64; 20] = [
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
    10_000_000_000_000_000_000,
];

/// convenience method to calculate the power when in base 10.
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub const fn pow_10(exp: u64) -> u64 {
    POW_10[exp as usize]
}

/// convenience method to calculate the power when in base 10.
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub const fn pow_10_usize(exp: usize) -> u64 {
    POW_10[exp]
}

/// convenience method to calculate the integer logarithm in base 10.
#[must_use]
pub const fn int_log_10(n: u64) -> u64 {
    // POW_10.iter().take_while(|&&d| d <= n).count() as _
    let mut i = 0;
    while POW_10[i] <= n {
        i += 1;
    }
    i as u64
}

// --- //

/// calculates an approximation of the square root
#[must_use]
pub const fn floor_sqrt(value: i64) -> i64 {
    exact_sqrt(value).0
}

/// calculates an approximation of the square root
#[must_use]
#[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
pub const fn floor_sqrt_u64(value: u64) -> u64 {
    exact_sqrt(value as i64).0 as u64
}

/// calculates an overestimation of the square root
#[must_use]
pub const fn ceil_sqrt(value: i64) -> i64 {
    let (root, remainder) = exact_sqrt(value);
    if remainder == 0 { root } else { root + 1 }
}

/// calculates an overestimation of the square root
#[must_use]
#[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
pub const fn ceil_sqrt_u64(value: u64) -> u64 {
    let (root, remainder) = exact_sqrt(value as i64);
    if remainder == 0 { root as u64 } else { root as u64 + 1 }
}

/// calculates an approximation of the square root
#[must_use]
pub const fn int_sqrt(value: i64) -> i64 {
    let (root, remainder) = exact_sqrt(value);
    // rounding to nearest integer
    if remainder > root { root + 1 } else { root }
}

/// calculates the square root composition: a square and a remainder
#[must_use]
pub const fn exact_sqrt(value: i64) -> (i64, i64) {
    // fast path for digit conversions on the default radix
    if value == DEFAULT_RADIX as i64 {
        return (3, 1);
    } else if value == 0 {
        return (0, 0);
    }

    // "place" starts at the highest power of four <= than the argument
    let leading = 0i64.leading_zeros() - value.leading_zeros();
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
#[must_use]
pub const fn floor_cube_root(value: i64) -> i64 {
    exact_root(value, 3).0
}

/// returns nth root of a number value (binary search algorithm)
#[must_use]
pub const fn exact_root(value: i64, n: i64) -> (i64, i64) {
    // set start and end for binary search as some powers of 2
    let leading = (63 - value.leading_zeros() as i64) / n;
    let (mut start, mut end) = (1 << leading, 1 << (leading + 1));
    loop {
        if end - start == 1 { break (start, value - pow(start, n)); }

        let mid = (start + end) >> 1;
        let error = value - pow(mid, n);
        if error.is_negative() { end = mid } else { start = mid }
    }
}

// --- //

/// power!
#[must_use]
#[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub const fn pow_u64(base: u64, exp: u64) -> u64 {
    if base == 10 {
        return POW_10[exp as usize];
    }
    pow(base as i64, exp as i64) as u64
}

/// power!
#[must_use]
#[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub const fn pow(base: i64, exp: i64) -> i64 {
    if base == 10 {
        return POW_10[exp as usize] as i64;
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
        return if exp % 2 == 0 { 1 } else { -1 };
    }
    if base == 2 {
        return 1 << exp;
    }
    if exp == 2 {
        return base * base;
    }
    squaring(base, exp)
}

const fn squaring(base: i64, exp: i64) -> i64 {
    let (mut sqr_base, mut sqr_exp, mut result) = (base, exp, 1);
    loop {
        if sqr_exp == 0 {
            return result;
        }
        if sqr_exp & 1 != 0 {
            result *= sqr_base;
        }
        sqr_base *= sqr_base;
        sqr_exp >>= 1;
    }
}

/// calculates the square af a given value
#[must_use]
pub const fn square(base: i64) -> i64 {
    base * base
}

/// calculates the square af a given value
#[must_use]
pub const fn square_u64(base: u64) -> u64 {
    base * base
}

/// verifies that a given value is a perfect square
#[must_use]
pub const fn is_square(value: i64) -> bool {
    let hex = value & 0xF; // last hexadecimal "digit" (has to be either 0, 1, 4 or 9)
    hex <= 9 && (hex == 0 || hex == 1 || hex == 4 || hex == 9) && exact_sqrt(value).1 == 0
}

/// calculates the cube of a given value
#[must_use]
pub const fn cube(base: i64) -> i64 {
    base * base * base
}

/// calculates the cube of a given value
#[must_use]
pub const fn cube_u64(base: u64) -> u64 {
    base * base * base
}

/// verifies that a given value is a perfect cube
#[must_use]
pub const fn is_cube(base: i64) -> bool {
    exact_root(base, 3).1 == 0
}

/// calculates the fourth power of a given value
#[must_use]
pub const fn fourth(base: i64) -> i64 {
    square(base) * square(base)
}

/// calculates the fourth power of a given value
#[must_use]
pub const fn fourth_u64(base: u64) -> u64 {
    square_u64(base) * square_u64(base)
}
