// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::factor::is_abundant;
use euler::algorithm::long::{arithmetic_sum, is_even};
use euler::Solver;

// A perfect number is a number for which the sum of its proper divisors is exactly equal to the number.
// For example, the sum of the proper divisors of 28 would be 1 + 2 + 4 + 7 + 14 = 28, which means that 28 is a perfect number.
// A number n is called deficient if the sum of its proper divisors is less than n and it is called abundant if this sum exceeds n.
// As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16, the smallest number that can be written as the sum of two abundant numbers is 24.
// By mathematical analysis, it can be shown that all integers greater than 28123 can be written as the sum of two abundant numbers.
// However, this upper limit cannot be reduced any further by analysis even though it is known that the greatest number that cannot be expressed as the sum of two abundant numbers is less than this limit.
// Find the sum of all the positive integers which cannot be written as the sum of two abundant numbers.

pub struct Solver023 {
    pub n: isize,
}

impl Default for Solver023 {
    fn default() -> Self {
        Solver023 { n: 28_123 }
    }
}

impl Solver for Solver023 {
    fn solve(&self) -> isize {
        let (mut abundant, mut list) = (vec![false; 1 + self.n as usize], Vec::with_capacity(self.n as _));
        arithmetic_sum(self.n) - (1..=self.n).filter(|&i| {
            // abundant numbers are even or multiples of 5. the boolean array is faster than a bit_set() as long is the size is known upfront.
            if (is_even(i) || i % 5 == 0) && is_abundant(i) {
                abundant[i as usize] |= true;
                list.push(i);
            }
            list.iter().take_while(|&&j| j <= i >> 1).any(|&j| abundant[(i - j) as usize])
        }).sum::<isize>()
    }
}
