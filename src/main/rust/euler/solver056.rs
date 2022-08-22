// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::long::{digits_sum, pow_10};
use euler::Solver;

// A googol (10^100) is a massive number: one followed by one-hundred zeros; 100^100 is almost unimaginably large: one followed by two-hundred zeros.
// Despite their size, the sum of the digits in each number is only 1.
//
// Considering natural numbers of the form, a^b , where a, b < 100, what is the maximum digital sum?

pub struct Solver056 {
    pub n: isize,
}

impl Default for Solver056 {
    fn default() -> Self {
        Solver056 { n: 100 }
    }
}

impl Solver for Solver056 {
    fn solve(&self) -> isize {
        // only test a fraction of the space, just the 10% biggest numbers!
        let (floor, ceil) = (9 * self.n / 10, self.n);
        (floor..ceil).map(|a| vectorized_power(a).skip((floor - 1) as _).take((ceil - floor) as _).map(|power| power.iter().map(digits_sum).sum()).max().unwrap()).max().unwrap()
    }
}

// --- //

///
fn vectorized_power(base: isize) -> impl Iterator<Item=Vec<isize>> {
    VectorizedPower { power: vec![1], base, threshold: pow_10(10) }
}

struct VectorizedPower {
    power: Vec<isize>,
    base: isize,
    threshold: isize,
}

impl Iterator for VectorizedPower {
    type Item = Vec<isize>;

    fn next(&mut self) -> Option<Self::Item> {
        let (base, threshold, mut carry) = (self.base, self.threshold, None);
        self.power.iter_mut().for_each(|cell| {
            let value = *cell * base + carry.unwrap_or_default();

            // adjust the buckets that grow beyond the ceiling value, carrying to next bucket
            carry = if value > threshold { Some(value / threshold) } else { None };
            *cell = if value > threshold { value % threshold } else { value };
        });

        // with a small cell_threshold values would probably need to split the carry into cells as well
        carry.iter().for_each(|&c| self.power.push(c));
        Some(self.power.clone())
    }
}
