// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::long::{DEFAULT_RADIX, incrementing_digits, is_palindrome_digits};
use euler::Solver;

// If we take 47, reverse and add, 47 + 74 = 121, which is palindromic.
// Not all numbers produce palindromes so quickly. For example,
// 349 + 943 = 1292,
// 1292 + 2921 = 4213
// 4213 + 3124 = 7337
//
// That is, 349 took three iterations to arrive at a palindrome.
//
// Although no one has proved it yet, it is thought that some numbers, like 196, never produce a palindrome.
// A number that never forms a palindrome through the reverse and add process is called a Lychrel number.
// Due to the theoretical nature of these numbers, and for the purpose of this problem, we shall assume that a number is Lychrel until proven otherwise.
// In addition you are given that for every number below ten-thousand, it will either (i) become a palindrome in less than fifty iterations, or, (ii) no one, with all the computing power that exists, has managed so far to map it to a palindrome.
// In fact, 10677 is the first number to be shown to require over fifty iterations before producing a palindrome: 4668731596684224866951378664 (53 iterations, 28-digits).
// Surprisingly, there are palindromic numbers that are themselves Lychrel numbers; the first example is 4994.
//
// How many Lychrel numbers are there below ten-thousand?
//
// NOTE: Wording was modified slightly on 24 April 2007 to emphasise the theoretical nature of Lychrel numbers.

const THRESHOLD: usize = 50;

pub struct Solver055 {
    pub n: isize,
}

impl Default for Solver055 {
    fn default() -> Self {
        Solver055 { n: 10_000 }
    }
}

impl Solver for Solver055 {
    fn solve(&self) -> isize {
        // sums a number in digit representation with it's reverse
        let mirror_sum = |a: &[isize]| {
            let (dim, mut carry, mut result) = (a.len(), 0, Vec::with_capacity(a.len() + 1));
            for i in 0..dim {
                let c = carry + a[i] + a[dim - i - 1];
                carry = c / DEFAULT_RADIX;
                result.push(c % DEFAULT_RADIX);
            }
            (carry != 0).then(|| result.push(carry));
            result
        };

        let is_lychrel = |mut value: Vec<_>| {
            (0..THRESHOLD).all(|_| {
                value = mirror_sum(&value);
                !is_palindrome_digits(&value)
            }).then_some(())
        };

        incrementing_digits(1).take(self.n as _).filter_map(is_lychrel).count() as _
    }
}
