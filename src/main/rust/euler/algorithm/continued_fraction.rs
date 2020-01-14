// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::mem::swap;

use euler::algorithm::long::{floor_sqrt, pow_10, square};

// multiplier / (√n - fractional) => a + (√n - b) / c
fn transform(n: isize, floor: isize, multiplier: isize, fractional: isize) -> (isize, isize, isize) {
    let c = (n - square(fractional)) / multiplier;
    let a = (fractional + floor) / c;
    let b = -fractional + c * a;
    (a, b, c)
}

/// length of cycle, by applying transformations until c == 1 (or a == 2 * floor)
pub fn cycle_len(n: isize) -> isize {
    let floor = floor_sqrt(n);
    if square(floor) == n {
        return 0;
    }
    let (mut multiplier, mut fractional) = (1, floor);
    (0..n).take_while(|_| {
        let (_, b, c) = transform(n, floor, multiplier, fractional);
        fractional = b;
        multiplier = c;
        multiplier != 1
    }).count() as isize + 1
}

/// continued fraction expansion, by applying transformations until c == 1 (or a == 2 * floor)
pub fn continued_expansion(n: isize) -> Vec<isize> {
    let floor = floor_sqrt(n);
    if square(floor) == n {
        return vec![0];
    }
    let (mut multiplier, mut fractional, mut expansion) = (1, floor, Vec::new());
    expansion.push(floor);
    (0..n).take_while(|_| {
        let (a, b, c) = transform(n, floor, multiplier, fractional);
        expansion.push(a);
        fractional = b;
        multiplier = c;
        multiplier != 1
    }).for_each(|_| ());
    expansion
}

// --- //

/// convenience function that calculates a += b * c
pub fn add_mul(a: &mut Vec<isize>, b: &[isize], c: isize, threshold: isize) {
    while a.len() < b.len() { a.push(0); }
    for i in 0..b.len() {
        a[i] += b[i] * c;
        if a[i] > threshold {
            if i == a.len() - 1 { a.push(a[i] / threshold) } else { a[i + 1] += a[i] / threshold }
            a[i] %= threshold;
        }
    }
}

/// Get the nth convergent (starting at 1)
pub fn convergent(value: isize, nth: isize) -> (Vec<isize>, Vec<isize>) {
    convergent_with_expansion(continued_expansion(value), nth)
}

/// Get the nth convergent (starting at 1)
pub fn convergent_with_expansion(expansion: Vec<isize>, nth: isize) -> (Vec<isize>, Vec<isize>) {
    let (f, threshold) = (|n| if n < expansion.len() { expansion[n] } else { expansion[1 + n % (expansion.len() - 1)] }, pow_10(15));
    let (mut n, mut d) = (vec![f(nth as usize - 1)], vec![1]);
    (0..nth - 1).rev().for_each(|i| {
        swap(&mut d, &mut n);
        add_mul(&mut n, &d, f(i as _), threshold);
    });
    (n, d)
}

/// Get the nth convergent (starting at 1)
pub fn convergent_with(f: fn(isize) -> isize, nth: isize) -> (Vec<isize>, Vec<isize>) {
    let (mut n, mut d, threshold) = (vec![f(nth - 1)], vec![1], pow_10(15));
    (0..nth - 1).rev().for_each(|i| {
        swap(&mut d, &mut n);
        add_mul(&mut n, &d, f(i), threshold);
    });
    (n, d)
}