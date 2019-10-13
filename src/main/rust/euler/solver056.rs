// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::long::{digits_sum, pow_10};
use euler::Solver;

// A googol (10^100) is a massive number: one followed by one-hundred zeros; 100^100 is almost unimaginably large: one followed by two-hundred zeros.
// Despite their size, the sum of the digits in each number is only 1.
//
// Considering natural numbers of the form, a^b , where a, b < 100, what is the maximum digital sum?

pub struct Solver056 {
    pub n: isize
}

impl Default for Solver056 {
    fn default() -> Self {
        Solver056 { n: 100 }
    }
}

impl Solver for Solver056 {
    fn solve(&self) -> isize {
        // Only test a fraction of the space, just the 10% biggest numbers!
        let (floor, ceil) = (9 * self.n / 10, self.n);
        let vec_digit_sum = |power: Vec<_>| power.iter().map(|&d| digits_sum(d)).sum::<_>();

        (floor..ceil).map(|a| vectorized_power(a).skip((floor - 1) as _).take((ceil - floor) as _).map(vec_digit_sum).max().unwrap()).max().unwrap()
    }
}

// --- //

struct VectorizedPower {
    power: Vec<isize>,
    base: isize,
}

fn vectorized_power(base: isize) -> VectorizedPower {
    VectorizedPower { power: vec![1], base }
}

impl Iterator for VectorizedPower {
    type Item = Vec<isize>;

    fn next(&mut self) -> Option<Self::Item> {
        let (mut carry, ceil) = (0, pow_10(10));
        for bucket in &mut self.power {
            let value = *bucket * self.base + carry;

            // Adjust the buckets that grow beyond the ceiling value, carrying to next bucket
            carry = if value > ceil { value / ceil } else { 0 };
            *bucket = if value > ceil { value % ceil } else { value };
        }
        if carry != 0 {
            // with a small ceiling values would probably need to split the carry into buckets
            self.power.push(carry)
        }
        Some(self.power.to_vec())
    }
}
