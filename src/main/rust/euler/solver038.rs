// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::long::{concatenation, from_digits, int_log_10, int_sqrt, is_pandigital, pow_10, to_digits};
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
        Solver038 { n: 9 }
    }
}

impl Solver for Solver038 {
    fn solve(&self) -> isize {
        let natural_product_concatenation = |value, size| {
            let (mut n, mut result) = (value, value);
            loop {
                if int_log_10(result) >= size {
                    return to_digits(result);
                }
                n += value;
                result = concatenation(result, n);
            }
        };

        // generate the natural product concatenation and check if it's a pandigital. find the first according to the most significant digit ordering
        let (product_concatenation, pandigital) = (|v| natural_product_concatenation(v, self.n), |d: &Vec<_>| d.len() == self.n as _ && is_pandigital(d));
        decrementing_by_most_significant(self.n, int_sqrt(self.n)).map(product_concatenation).find(pandigital).map_or(0, from_digits)
    }
}

// --- //

/// creates an iterator that returns elements ordered by most significant digit: 999 ... 900 -> 99 ... 90 -> 9 -> 899 ... 800 -> 89 .. 80 -> 8 -> 799
fn decrementing_by_most_significant(value: isize, scale: isize) -> impl Iterator<Item=isize> {
    DecrementingByMostSignificant { range: Box::new(0..0), size: 0, value: value + 1, scale}
}

struct DecrementingByMostSignificant {
    range: Box<dyn Iterator<Item=isize>>,
    size: isize,
    value: isize,
    scale: isize,
}

impl Iterator for DecrementingByMostSignificant {
    type Item = isize;

    fn next(&mut self) -> Option<isize> {
        self.range.next().or_else(|| {
            if self.size > 0 {
                self.size -= 1;
            } else {
                self.size = self.scale;
                self.value -= 1;
            }
            if self.value > 0 {
                self.range = Box::new((self.value * pow_10(self.size)..(self.value + 1) * pow_10(self.size)).rev());
                self.range.next().filter(|&v| v > 1)
            } else {
                None
            }
        })
    }
}
