// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::mem::swap;

use euler::algorithm::long::{floor_sqrt, pow_10, square, exact_sqrt};

// multiplier / (√n - fractional) => a + (√n - b) / c
fn transform(n: isize, floor: isize, multiplier: isize, fractional: isize) -> (isize, isize, isize) {
    let c = (n - square(fractional)) / multiplier;
    let a = (fractional + floor) / c;
    let b = -fractional + c * a;
    (a, b, c)
}

/// length of cycle of sqrt(n), by applying transformations until c == 1 (or a == 2 * floor)
pub fn continued_expansion_sqrt_cycle_len(n: isize) -> isize {
    // this is an optimization of continued_expansion_sqrt(n) that does not store values
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

/// continued fraction expansion of sqrt(n), by applying transformations until c == 1 (or a == 2 * floor)
pub fn continued_expansion_sqrt(n: isize) -> Vec<isize> {
    let (floor, remainder) = exact_sqrt(n);
    if remainder == 0 {
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

/// continued fraction expansion of rational n/d, by applying Eucledean algorithm
pub fn continued_expansion_rational(mut n: isize, mut d: isize) -> Vec<isize> {
    let mut expansion = vec![];
    while d != 0 {
        expansion.push(n / d);
        swap(&mut n, &mut d);
        d %= n;
    }
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

// --- //

/// closure that approximates a real value by iterating the convergent
pub struct Convergent<'a> {
    f: Box<dyn Fn(usize) -> Option<isize> + 'a>,
    previous: (Vec<isize>, Vec<isize>),
    last: (Vec<isize>, Vec<isize>),
    i: usize,
    threshold: isize,
}

pub fn convergent_with<'a>(f: Box<dyn Fn(usize) -> Option<isize>>) -> Convergent<'a> {
    Convergent { f, previous: (vec![0], vec![1]), last: (vec![1], vec![0]), i: 0, threshold: pow_10(15) }
}

pub fn convergent_with_expansion(expansion: &[isize]) -> Convergent {
    let f = move |n| if n < expansion.len() { Some(expansion[n]) } else { None };
    Convergent { f: Box::new(f), previous: (vec![0], vec![1]), last: (vec![1], vec![0]), i: 0, threshold: pow_10(15) }
}

pub fn convergent_cyclic(expansion: &[isize]) -> Convergent {
    let f = move |n| Some(expansion[if n < expansion.len() { n } else { 1 + n % (expansion.len() - 1) }]);
    Convergent { f: Box::new(f), previous: (vec![0], vec![1]), last: (vec![1], vec![0]), i: 0, threshold: pow_10(15) }
}

impl<'a> Iterator for Convergent<'a> {
    type Item = (Vec<isize>, Vec<isize>);

    fn next(&mut self) -> Option<Self::Item> {
        self.i += 1;
        if let Some(value) = (*self.f)(self.i - 1) {
            add_mul(&mut self.previous.0, &self.last.0, value, self.threshold);
            add_mul(&mut self.previous.1, &self.last.1, value, self.threshold);
            swap(&mut self.last, &mut self.previous);
            Some((self.last.0.to_vec(), self.last.1.to_vec()))
        } else {
            None
        }
    }
}
