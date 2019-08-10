// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::iter::Rev;
use std::ops::Range;

use euler::algorithm::long::{concatenation, from_digits, int_sqrt, is_pandigital, pow_10, to_digits};
use euler::Solver;

// Take the number 192 and multiply it by each of 1, 2, and 3:
// 192 × 1 = 192
// 192 × 2 = 384
// 192 × 3 = 576
// By concatenating each product we get the 1 to 9 pandigital, 192384576. We will call 192384576 the concatenated product of 192 and (1,2,3)
// The same can be achieved by starting with 9 and multiplying by 1, 2, 3, 4, and 5, giving the pandigital, 918273645, which is the concatenated product of 9 and (1,2,3,4,5).
// What is the largest 1 to 9 pandigital 9-digit number that can be formed as the concatenated product of an integer with (1,2, ... , n) where n > 1?

pub struct Solver038 {
    pub n: isize,
}

impl Default for Solver038 {
    fn default() -> Self {
        Solver038 {
            n: 9
        }
    }
}

impl Solver for Solver038 {
    fn solve(&self) -> isize {
        // generate the natural product concatenation and check if it's a pandigital. find the first according to the most significant digit ordering
        let (product_concatenation, pandigital) = (|v| natural_product_concatenation(v, self.n), |d: &Vec<isize>| d.len() == self.n as usize && is_pandigital(d));
        decrementing_by_most_significant(self.n, int_sqrt(self.n)).map(product_concatenation).find(pandigital).map_or(0, |d| from_digits(&d))
    }
}

fn natural_product_concatenation(value: isize, size: isize) -> Vec<isize> {
    let (mut n, mut digits) = (value, to_digits(value));
    loop {
        if digits.len() >= size as usize {
            return digits;
        }
        n += value;
        digits = concatenation(&mut digits, &mut to_digits(n));
    }
}

// --- //

struct DecrementingByMostSignificant {
    range: Rev<Range<isize>>,
    size: Rev<Range<isize>>,
    value: isize,
    scale: isize,
}

/// Stream that returns elements ordered by most significant digit: 999 ... 900 -> 99 ... 90 -> 9 -> 899 ... 800 -> 89 .. 80 -> 8 -> 799
// actually, for the purpose of this problem, does not return single digit values
fn decrementing_by_most_significant(value: isize, scale: isize) -> DecrementingByMostSignificant {
    DecrementingByMostSignificant { range: decrementing_range(value, scale), size: (1..scale).rev(), value, scale }
}

fn decrementing_range(value: isize, scale: isize) -> Rev<Range<isize>> {
    (value * pow_10(scale)..(value + 1) * pow_10(scale)).rev()
}

impl Iterator for DecrementingByMostSignificant {
    type Item = isize;

    fn next(&mut self) -> Option<isize> {
        self.range.next().or_else(|| {
            let next_size = self.size.next().unwrap_or_else(|| {
                self.value -= 1;
                self.size = (1..self.scale).rev();
                self.size.next().unwrap()
            });
            if self.value != 0 {
                self.range = decrementing_range(self.value, next_size);
                self.range.next()
            } else {
                None
            }
        })
    }
}
