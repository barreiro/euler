// COPYRIGHT (C) 2022 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::convert::{TryFrom, TryInto};
use std::iter::{FromIterator, Map};
use std::num::TryFromIntError;
use std::ops::AddAssign;

use algorithm::cast::Cast;
use algorithm::combinatorics::permutations_of_set_with;
use algorithm::root::{int_log_10, pow, pow_10};

/// constant that defines the default base for conversions to and from digits
pub const DEFAULT_RADIX: u8 = 10;

pub type Digit = u8;

/// Digit representation of a number (from most to least significant)
#[derive(Clone)]
pub struct Digits {
    digits: Vec<u8>,
    radix: u8,
}

impl Digits {
    /// verifies if it has all the digits one and only once (excluding zero)
    #[must_use]
    pub fn is_pandigital(&self) -> bool {
        self.len() <= self.radix as usize && (1..=self.len()).filter_map(|value| u8::try_from(value).ok()).all(|value| self.digits.contains(&value))
    }

    /// verifies if reads the same both ways
    #[must_use]
    pub fn is_palindrome(&self) -> bool {
        (0..=self.digits.len() >> 1).all(|i| self.digits[i] == self.digits[self.digits.len() - i - 1])
    }

    /// checks if a value has repeated digits
    #[must_use]
    pub fn is_unique(&self) -> bool {
        let mut digits = self.digits.clone();
        digits.sort_unstable();
        (1..digits.len()).all(|i| digits[i] != digits[i - 1])
    }

    /// gets the n-th digit (from least to most significant)
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&u8> {
        self.digits.get(index)
    }

    /// gets the last digit (least significant)
    #[must_use]
    pub fn last(&self) -> Option<&u8> {
        self.digits.last()
    }

    /// gets the first digit (most significant)
    #[must_use]
    pub fn first(&self) -> Option<&u8> {
        self.digits.first()
    }

    /// number of digits
    #[must_use]
    pub fn len(&self) -> usize {
        self.digits.len()
    }

    /// has digits
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.digits.len() == 0
    }

    /// contains a certain digit
    #[must_use]
    pub fn contains(&self, digit: &u8) -> bool {
        self.digits.contains(digit)
    }

    /// reverse the digits order
    pub fn reverse(&mut self) {
        self.digits.reverse();
    }

    /// rotate the digits to the left
    pub fn rotate_left(&mut self) {
        self.digits.rotate_left(1);
    }

    /// creates a fingerprint of digits, to identify if one is permutation of another
    #[must_use]
    pub fn to_fingerprint(mut self) -> u64 {
        self.digits.sort_unstable();
        from_raw_digits(&self.digits)
    }

    /// the value that's represented by these digits
    #[must_use]
    pub fn value(&self) -> u64 {
        from_raw_digits(&self.digits)
    }

    /// finds a digit that is repeated `repetitions` times
    #[must_use]
    #[allow(clippy::naive_bytecount)]
    pub fn find_repeated(&self, repetitions: usize) -> Option<u8> {
        (0..DEFAULT_RADIX).find(|&i| self.digits.iter().filter(|&&d| d == i).count() >= repetitions)
    }

    /// an iterator of permutations that match a given predicate
    pub fn permutations_with<F, R>(&self, predicate: F) -> impl Iterator<Item=R> where F: Fn(&[u8]) -> Option<R> {
        let mut set = self.digits.clone();
        set.sort_unstable();
        permutations_of_set_with(set, predicate)
    }

    /// returns an iterator on the digits
    #[must_use]
    pub fn iter(&self) -> impl DoubleEndedIterator<Item=&u8> {
        self.digits.iter()
    }
}

fn to_raw_digits_radix(mut value: u64, radix: u8) -> Vec<u8> {
    let mut digits = Vec::with_capacity(12);
    while value >= u64::from(radix) {
        digits.push(u8::try_from(value.rem_euclid(u64::from(radix))).expect("Remainder should be smaller than radix"));
        value /= u64::from(radix);
    }
    digits.push(u8::try_from(value).expect("Final remainder should be smaller than radix"));
    digits
}

/// the value of a collection of digits
#[must_use]
pub const fn from_raw_digits(digits: &[u8]) -> u64 {
    from_raw_digits_radix(digits, DEFAULT_RADIX)
}

#[allow(clippy::cast_sign_loss)]
const fn from_raw_digits_radix(digits: &[u8], radix: u8) -> u64 {
    let (mut result, mut i, base_10) = (0, 0, radix == DEFAULT_RADIX);
    while i < digits.len() {
        result += digits[i] as u64 * if base_10 { pow_10(i as u64) } else { pow(radix as i64, i as i64) as u64 };
        i += 1;
    }
    result
}

// --- //

/// creates a digit from a value anda a given radix
impl From<(u64, u8)> for Digits {
    fn from((value, radix): (u64, u8)) -> Self {
        Self { digits: to_raw_digits_radix(value, radix), radix }
    }
}

impl From<Vec<u8>> for Digits {
    fn from(digits: Vec<u8>) -> Self {
        Self { digits, radix: DEFAULT_RADIX }
    }
}

impl From<&[u8]> for Digits {
    fn from(digits: &[u8]) -> Self {
        Self { digits: digits.to_vec(), radix: DEFAULT_RADIX }
    }
}

impl From<u64> for Digits {
    fn from(value: u64) -> Self {
        (value, DEFAULT_RADIX).into()
    }
}

impl From<i64> for Digits {
    fn from(value: i64) -> Self {
        (value.as_u64(), DEFAULT_RADIX).into()
    }
}

impl From<Digits> for u64 {
    fn from(d: Digits) -> Self {
        from_raw_digits_radix(&d.digits, DEFAULT_RADIX)
    }
}

impl TryFrom<Digits> for i64 {
    type Error = TryFromIntError;

    fn try_from(d: Digits) -> Result<Self, Self::Error> {
        from_raw_digits_radix(&d.digits, DEFAULT_RADIX).try_into()
    }
}

impl FromIterator<u64> for Digits {
    fn from_iter<T: IntoIterator<Item=u64>>(iter: T) -> Self {
        let mut value = 0;
        for v in iter {
            value = concatenation(value, v);
        }
        value.into()
    }
}

impl FromIterator<u8> for Digits {
    fn from_iter<T: IntoIterator<Item=u8>>(iter: T) -> Self {
        Vec::from_iter(iter).into()
    }
}

impl IntoIterator for Digits {
    type Item = u64;
    type IntoIter = Map<std::vec::IntoIter<u8>, Box<dyn FnMut(u8) -> u64>>;

    fn into_iter(self) -> Self::IntoIter {
        self.digits.into_iter().map(Box::new(u64::from))
    }
}

// --- //

/// an iterator on the digits of a given value
pub fn digits_iter(value: u64) -> impl Iterator<Item=u64> {
    DigitsIterator { value, radix: DEFAULT_RADIX }
}

struct DigitsIterator {
    value: u64,
    radix: u8,
}

impl Iterator for DigitsIterator {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.value == 0 {
            None
        } else {
            let ret = self.value % u64::from(self.radix);
            self.value /= u64::from(self.radix);
            Some(ret)
        }
    }
}

// --- //

impl AddAssign for Digits {
    fn add_assign(&mut self, rhs: Self) {
        assert_eq!(self.radix, rhs.radix);
        let mut carry = 0;
        for i in 0..self.digits.len() {
            let c = self.digits[i] + rhs.digits.get(i).unwrap_or(&0) + carry;
            carry = c / self.radix;
            self.digits[i] = c % self.radix;
        }
        for i in self.digits.len()..rhs.digits.len() {
            let c = rhs.digits[i] + carry;
            carry = c / self.radix;
            self.digits.push(c % self.radix);
        }
        (carry != 0).then(|| self.digits.push(carry));
    }
}

// --- //

/// concatenate two values
#[must_use]
pub const fn concatenation(one: u64, two: u64) -> u64 {
    one * pow_10(int_log_10(two)) + two
}

/// checks if two values are permutations of one another, i.e. have the same digits but it different order
#[must_use]
pub fn is_permutation(a: u64, b: u64) -> bool {
    if a % 9 == b % 9 {
        let (mut digits_a, mut digits_b) = (Digits::from(a), Digits::from(b));
        if digits_a.digits.len() == digits_b.digits.len() {
            digits_a.digits.sort_unstable();
            digits_b.digits.sort_unstable();
            return (0..digits_a.digits.len()).all(|i| digits_a.digits[i] == digits_b.digits[i]);
        }
    }
    false
}

/// calculates the sum of the digits of a given value
#[must_use]
#[allow(clippy::cast_possible_wrap)]
pub const fn digits_sum(value: u64) -> i64 {
    let (mut sum, mut v, radix) = (0, value, DEFAULT_RADIX as u64);
    while v >= radix {
        sum += v % radix;
        v /= radix;
    }
    (sum + v) as i64
}

/// calculates the sum of the squares digits of a given value
#[must_use]
pub const fn digits_square_sum(mut value: u64) -> u64 {
    let (mut sum, radix) = (0, DEFAULT_RADIX as u64);
    while value >= radix {
        let remainder = value % radix;
        sum += remainder * remainder;
        value /= radix;
    }
    sum + value * value
}

/// trim the first `n` digits
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub const fn skip_digits(value: u64, n: usize) -> u64 {
    last_digits(value, int_log_10(value) as usize + 1 - n)
}

/// the last `n` digits
#[must_use]
pub const fn last_digits(value: u64, n: usize) -> u64 {
    value % pow_10(n as u64)
}

/// the first `n` digits
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub const fn first_digits(value: u64, n: usize) -> u64 {
    let int_log = int_log_10(value) as usize; // alternative to max() function for const
    value / pow_10(((if int_log > n { int_log } else { n }) - n) as u64)
}

/// the `n`th digit
#[must_use]
pub const fn nth_digit(value: u64, n: u64) -> u64 {
    value / pow_10(int_log_10(value) - n - 1) % DEFAULT_RADIX as u64
}

// --- //

/// iterator on palindromes (numbers that read the same in both ways)
pub fn palindromes() -> impl Iterator<Item=u64> {
    Palindromes { digits: vec![0] }
}

struct Palindromes {
    pub digits: Vec<u8>,
}

impl Iterator for Palindromes {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.palindrome_increase() && !self.palindrome_rotate() {
            self.palindrome_expand();
        }
        Some(from_raw_digits(&self.digits))
    }
}

impl Palindromes {
    fn palindrome_increase(&mut self) -> bool {
        let middle = self.digits.len() / 2;
        if self.digits[middle] >= 9 {
            return false;
        }
        if self.digits.len() % 2 == 0 {
            self.digits[middle - 1] += 1;
        }
        self.digits[middle] += 1;
        true
    }

    fn palindrome_rotate(&mut self) -> bool {
        let size = self.digits.len();
        for i in size / 2 + 1..size {
            self.digits[i - 1] = 0;
            self.digits[size - i] = 0;

            if self.digits[i] != 9 {
                self.digits[i] += 1;
                self.digits[size - i - 1] += 1;
                return true;
            }
        }
        false
    }

    fn palindrome_expand(&mut self) {
        let size = self.digits.len();
        self.digits = vec![0; size + 1];
        self.digits[0] = 1;
        self.digits[size] = 1;
    }
}

// --- //

/// provides an iterator of numbers in digit format
pub fn incrementing_digits() -> impl Iterator<Item=Digits> {
    incrementing_digits_from(0)
}

/// provides an iterator of numbers in digit format, starting at an initial value
pub fn incrementing_digits_from(initial: u64) -> impl Iterator<Item=Digits> {
    IncrementingDigits { array: to_raw_digits_radix(initial, DEFAULT_RADIX) }
}

struct IncrementingDigits {
    array: Vec<u8>,
}

impl Iterator for IncrementingDigits {
    type Item = Digits;

    fn next(&mut self) -> Option<Self::Item> {
        let digits = Digits { digits: self.array.clone(), radix: DEFAULT_RADIX };
        if !self.increase() && !self.rotate() {
            self.expand();
        }
        Some(digits)
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
