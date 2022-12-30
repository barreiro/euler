// COPYRIGHT (C) 2022 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::ops::{Add, Div, Mul, Sub};
use algorithm::cast::{Cast, UCast};

use algorithm::combinatorics::combinations_with;
use algorithm::digits::from_raw_digits;
use Solver;

/// By using each of the digits from the set, `{1, 2, 3, 4}`, exactly once, and making use of the four arithmetic operations `(+, −, *, /)` and brackets/parentheses, it is possible to form different positive integer targets.
/// For example:
///      ` 8 = (4 * (1 + 3)) / 2`
///      `14 = 4 * (3 + 1 / 2)`
///      `19 = 4 * (2 + 3) − 1`
///      `36 = 3 * 4 * (2 + 1)`
///
/// Note that concatenations of the digits, like `12 + 34`, are not allowed.
///
/// Using the set, `{1, 2, 3, 4}`, it is possible to obtain thirty-one different target numbers of which 36 is the maximum, and each of the numbers `1` to `28` can be obtained before encountering the first non-expressible number.
/// Find the set of four distinct digits, `a < b < c < d`, for which the longest set of consecutive positive integers, `1` to `n`, can be obtained, giving your answer as a string: `abcd`.
pub struct Solver093 {
    pub n: usize,
}

impl Default for Solver093 {
    fn default() -> Self {
        Self { n: 4 }
    }
}

impl Solver for Solver093 {
    fn solve(&self) -> i64 {
        let consecutive_expansion = |set: &[u8]| expansion(set).iter().enumerate().take_while(|(n, &e)| n + 1 == e.as_u64().as_usize()).count();
        combinations_with((1..=9).collect::<Vec<_>>(), self.n, |c| Some(c.to_vec())).max_by_key(|d| consecutive_expansion(d)).map(|d| from_raw_digits(&d)).as_i64()
    }
}

// --- //

// converts from a set of integers into all the possible target results
fn expansion(set: &[u8]) -> Vec<i64> {
    let rationals = set.iter().map(Rational::from).collect::<Vec<_>>();
    let mut result = rational_expansion(&rationals).iter().filter_map(Rational::as_i64).filter(|&x| x > 0).collect::<Vec<_>>();
    result.sort_unstable();
    result.dedup();
    result
}

fn rational_expansion(set: &[Rational]) -> Vec<Rational> {
    if set.len() == 1 {
        set.to_vec()
    } else if set.len() == 2 {
        rational_product(&[set[0]], &[set[1]])
    } else {
        // split into all possible partitions (by increasing the size len) and recurse
        let mut result = vec![];
        for len in 1..=set.len() / 2 {
            combinations_with(set.to_vec(), len, |comb| {
                let mut working_set = set.to_vec();
                working_set.retain(|w| !comb.contains(w));
                Some((comb.to_vec(), working_set))
            }).for_each(|(p1, p2)| result.append(&mut rational_product(&rational_expansion(&p1), &rational_expansion(&p2))));
            // memoization in the expansion showed no significant advantage
        }
        result
    }
}

// cartesian product of two sets of rationals (over all the possible operations - note the symmetry)
fn rational_product(a: &[Rational], b: &[Rational]) -> Vec<Rational> {
    let mut result = Vec::with_capacity(a.len() * b.len() * 6);
    for &a in a {
        for &b in b {
            result.append(&mut vec![a + b, a * b, a - b, a / b, b - a, b / a]);
        }
    }
    result
}

// --- //

#[derive(Clone, Copy, PartialEq)]
struct Rational {
    n: i64,
    d: i64,
}

impl From<&u8> for Rational {
    fn from(&n: &u8) -> Self {
        Self { n: i64::from(n), d: 1 }
    }
}

impl Add for Rational {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { n: self.n * rhs.d + rhs.n * self.d, d: self.d * rhs.d }
    }
}

impl Sub for Rational {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { n: self.n * rhs.d - rhs.n * self.d, d: self.d * rhs.d }
    }
}

impl Mul for Rational {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self { n: self.n * rhs.n, d: self.d * rhs.d }
    }
}

impl Div for Rational {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self { n: self.n * rhs.d, d: self.d * rhs.n }
    }
}

impl Rational {
    const fn as_i64(&self) -> Option<i64> {
        if self.d == 1 {
            Some(self.n)
        } else if self.d != 0 && self.n % self.d == 0 {
            Some(self.n / self.d)
        } else {
            None
        }
    }
}
