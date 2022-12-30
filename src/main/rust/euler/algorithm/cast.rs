// COPYRIGHT (C) 2022 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::convert::TryFrom;

use algorithm::digits::{DEFAULT_RADIX, Digits};

pub trait Cast {
    fn as_i64(&self) -> i64;
    fn as_u64(&self) -> u64;
}

pub trait UCast {
    fn as_usize(&self) -> usize;
}


// --- //

/// safe and easy cast from char
impl Cast for char {
    fn as_i64(&self) -> i64 {
        if *self >= '0' && *self <= '9' {
            i64::from(*self as u32 - '0' as u32)
        } else {
            i64::from((*self >= 'A').then_some(1 + *self as u32 - 'A' as u32).or_else(|| self.to_digit(u32::from(DEFAULT_RADIX))).unwrap_or_default())
        }
    }
    fn as_u64(&self) -> u64 {
        if *self >= '0' && *self <= '9' {
            u64::from(*self as u32 - '0' as u32)
        } else {
            u64::from((*self >= 'A').then_some(1 + *self as u32 - 'A' as u32).or_else(|| self.to_digit(u32::from(DEFAULT_RADIX))).unwrap_or_default())
        }
    }
}

/// safe and easy cast from Option<u64>
impl Cast for Option<u64> {
    fn as_i64(&self) -> i64 {
        i64::try_from(self.unwrap_or_default()).unwrap()
    }
    fn as_u64(&self) -> u64 {
        self.unwrap_or_default()
    }
}

/// safe and easy cast from Option<i64>
impl Cast for Option<i64> {
    fn as_i64(&self) -> i64 {
        self.unwrap_or_default()
    }
    fn as_u64(&self) -> u64 {
        u64::try_from(self.unwrap_or_default()).unwrap()
    }
}

/// safe and easy cast from Option<Digits>
impl Cast for Option<Digits> {
    fn as_i64(&self) -> i64 {
        i64::try_from(self.as_ref().map_or(0, Digits::value)).unwrap_or_default()
    }
    fn as_u64(&self) -> u64 {
        self.as_ref().map_or(0, Digits::value)
    }
}

/// safe and easy cast from f64
impl Cast for f64 {
    fn as_i64(&self) -> i64 {
        unsafe{ self.to_int_unchecked() }
    }
    fn as_u64(&self) -> u64 {
        unsafe{ self.to_int_unchecked() }
    }
}

/// safe and easy cast from u64
impl Cast for i64 {
    fn as_i64(&self) -> i64 {
        *self
    }
    fn as_u64(&self) -> u64 {
        u64::try_from(*self).unwrap()
    }
}

/// safe and easy cast from u64
impl Cast for u64 {
    fn as_i64(&self) -> i64 {
        i64::try_from(*self).unwrap()
    }
    fn as_u64(&self) -> u64 {
        *self
    }
}

/// safe and easy cast from u64
impl UCast for u64 {
    fn as_usize(&self) -> usize {
        usize::try_from(*self).unwrap_or_default()
    }
}

/// safe and easy cast from usize
impl Cast for usize {
    fn as_i64(&self) -> i64 {
        i64::try_from(*self).unwrap()
    }
    fn as_u64(&self) -> u64 {
        *self as u64
    }
}

// --- //

#[must_use]
pub fn to_i64(u: u64) -> i64 {
    i64::try_from(u).unwrap_or_default()
}

#[must_use]
pub fn to_u64(u: i64) -> u64 {
    u64::try_from(u).unwrap_or_default()
}

#[must_use]
pub fn map_char_as_i64(c: char) -> i64 {
    c.as_i64()
}
