// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::mem::swap;

use algorithm::root::{exact_sqrt, floor_sqrt, pow_10, square};

// multiplier / (√n - fractional) => a + (√n - b) / c
const fn transform(n: i64, floor: i64, multiplier: i64, fractional: i64) -> (i64, i64, i64) {
    let c = (n - square(fractional)) / multiplier;
    let a = (fractional + floor) / c;
    let b = -fractional + c * a;
    (a, b, c)
}

/// length of cycle of `sqrt(n)`, by applying transformations until `c == 1` (or `a == 2 * floor`)
#[must_use]
pub fn continued_expansion_sqrt_cycle_len(n: i64) -> usize {
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
    }).count() + 1
}

/// continued fraction expansion of `sqrt(n)`, by applying transformations until `c == 1` (or `a == 2 * floor`)
#[must_use]
pub fn continued_expansion_sqrt(n: i64) -> Vec<i64> {
    let (floor, remainder) = exact_sqrt(n);
    if remainder == 0 {
        vec![0]
    } else {
        let (mut multiplier, mut fractional, mut expansion) = (1, floor, vec![floor]);
        (0..n).take_while(|_| {
            let (a, b, c) = transform(n, floor, multiplier, fractional);
            (fractional, multiplier) = (b, c);
            expansion.push(a);
            multiplier != 1
        }).last();
        expansion
    }
}

/// continued fraction expansion of rational `n/d`, by applying Eucledean algorithm
#[must_use]
pub fn continued_expansion_rational(mut n: u64, mut d: u64) -> Vec<u64> {
    let mut expansion = vec![];
    while d != 0 {
        expansion.push(n / d);
        swap(&mut n, &mut d);
        d %= n;
    }
    expansion
}

// --- //

/// convenience function that calculates `a += b * c`
pub fn add_mul(a: &mut Vec<u64>, b: &[u64], c: u64, threshold: u64) {
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

pub fn convergent_with(f: Box<dyn Fn(usize) -> Option<u64>>) -> impl Iterator<Item=(Vec<u64>, Vec<u64>)> {
    Convergent { f, previous: (vec![0], vec![1]), last: (vec![1], vec![0]), i: 0, threshold: pow_10(15) }
}

pub fn convergent_with_expansion(expansion: &[u64]) -> impl Iterator<Item=(Vec<u64>, Vec<u64>)> + '_ {
    let f = Box::new(move |n| if n < expansion.len() { Some(expansion[n]) } else { None });
    Convergent { f, previous: (vec![0], vec![1]), last: (vec![1], vec![0]), i: 0, threshold: pow_10(15) }
}

pub fn convergent_cyclic(expansion: &[u64]) -> impl Iterator<Item=(Vec<u64>, Vec<u64>)> + '_ {
    let f = Box::new(move |n| Some(expansion[if n < expansion.len() { n } else { 1 + n % (expansion.len() - 1) }]));
    Convergent { f, previous: (vec![0], vec![1]), last: (vec![1], vec![0]), i: 0, threshold: pow_10(15) }
}

// closure that approximates a real value by iterating the convergent
struct Convergent<'a> {
    f: Box<dyn Fn(usize) -> Option<u64> + 'a>,
    previous: (Vec<u64>, Vec<u64>),
    last: (Vec<u64>, Vec<u64>),
    i: usize,
    threshold: u64,
}

impl<'a> Iterator for Convergent<'a> {
    type Item = (Vec<u64>, Vec<u64>);

    fn next(&mut self) -> Option<Self::Item> {
        (*self.f)(self.i).map(|value| {
            self.i += 1;
            add_mul(&mut self.previous.0, &self.last.0, value, self.threshold);
            add_mul(&mut self.previous.1, &self.last.1, value, self.threshold);
            swap(&mut self.last, &mut self.previous);
            (self.last.0.clone(), self.last.1.clone())
        })
    }
}
