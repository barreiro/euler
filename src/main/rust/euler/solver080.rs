// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use crate::algorithm::cast::{Cast, to_u64};
use crate::algorithm::digits::digits_sum_i64;
use crate::algorithm::long::GetAndIncrement;
use crate::algorithm::root::{floor_sqrt, int_log_10, is_square, pow_10};
use crate::Solver;

const DIGITS: u64 = 100;

#[allow(clippy::cast_possible_wrap)]
const THRESHOLD: i64 = pow_10(15) as i64;

/// It is well known that if the square root of a natural number is not an integer, then it is irrational.
/// The decimal expansion of such square roots is infinite without any repeating pattern at all.
///
/// The square root of two is `1.41421356237309504880…`, and the digital sum of the first one hundred decimal digits is `475`.
///
/// For the first one hundred natural numbers, find the total of the digital sums of the first one hundred decimal digits for all the irrational square roots.
pub struct Solver080 {
    pub n: i64,
}

impl Default for Solver080 {
    fn default() -> Self {
        Self { n: 100 }
    }
}

impl Solver for Solver080 {
    fn problem_name(&self) -> &str { "Square root digital expansion" }

    fn solve(&self) -> i64 {
        // "Square roots by subtraction" by Frazer Jarvis (http://www.afjarvis.staff.shef.ac.uk/maths/jarvisspec02.pdf)
        (2..=self.n).filter(|&n| !is_square(n)).map(|n| {
            let (mut a, mut b, mut i) = (vec![5 * n], vec![5], DIGITS - int_log_10(floor_sqrt(n).as_u64()));
            loop {
                if less(&a, &b) { // first branch fixes a digit of the root in b
                    if i == 0 { break; }
                    i -= 1;
                    insert_zero(&mut b);
                    mul_scalar(&mut a, 100);
                } else {
                    sub(&mut a, &b);
                    add_scalar(&mut b, 10);
                }
            }
            b.into_iter().map(to_u64).map(digits_sum_i64).sum::<i64>() - 5 // b ends with an extra '5'
        }).sum()
    }
}

// -- //

// convenience function that compares two vector numbers
fn less(a: &[i64], b: &[i64]) -> bool {
    if a.len() == b.len() {
        (0..a.len()).rev().find(|&i| a[i] != b[i]).is_some_and(|i| a[i] < b[i])
    } else {
        a.len() < b.len()
    }
}

// add a zero just before the final digit (which will always be '5')
fn insert_zero(a: &mut Vec<i64>) {
    a[0] /= 10;
    a[0] *= 10;
    mul_scalar(a, 10);
    a[0] += 5;
}

// convenience function that calculates a *= c where c is *not* a vector number
fn mul_scalar(a: &mut Vec<i64>, c: i64) {
    a.iter_mut().for_each(|i| *i *= c);
    for i in 0..a.len() {
        if a[i] >= THRESHOLD {
            if i == a.len() - 1 { a.push(a[i] / THRESHOLD) } else { a[i + 1] += a[i] / THRESHOLD }
            a[i] %= THRESHOLD;
        }
    }
}

// convenience function that calculates a -= b
fn sub(a: &mut Vec<i64>, b: &[i64]) {
    while a.len() < b.len() { a.push(0); }
    for i in 0..b.len() {
        a[i] -= b[i];
        if a[i] < 0 {
            a[i] += THRESHOLD;
            a[i + 1] -= 1;
        } else if a[i] >= THRESHOLD {
            if i == a.len() - 1 { a.push(a[i] / THRESHOLD) } else { a[i + 1] += a[i] / THRESHOLD }
            a[i] %= THRESHOLD;
        }
    }
    // need to normalize to be able to compare based on length
    while a.last() == Some(&0) {
        a.pop();
    }
}

// convenience function that calculates a += c where c is *not* a vector number
fn add_scalar(a: &mut Vec<i64>, c: i64) {
    a[0] += c;
    let mut i = 0;
    while a[i] >= THRESHOLD {
        if i == a.len() - 1 { a.push(a[i] / THRESHOLD) } else { a[i + 1] += a[i] / THRESHOLD }
        a[i.get_and_increment()] %= THRESHOLD;
    }
}
