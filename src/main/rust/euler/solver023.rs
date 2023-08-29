// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::cast::Cast;
use algorithm::filter::is_abundant_usize;
use algorithm::long::arithmetic_sum;
use Solver;

/// A perfect number is a number for which the sum of its proper divisors is exactly equal to the number.
///
/// For example, the sum of the proper divisors of `28` would be `1 + 2 + 4 + 7 + 14 = 28`, which means that `28` is a perfect number.
///
/// A number `n` is called deficient if the sum of its proper divisors is less than `n` and it is called abundant if this sum exceeds `n`.
///
/// As `12` is the smallest abundant number, `1 + 2 + 3 + 4 + 6 = 16`, the smallest number that can be written as the sum of two abundant numbers is `24`.
///
/// By mathematical analysis, it can be shown that all integers greater than `28123` can be written as the sum of two abundant numbers.
///
/// However, this upper limit cannot be reduced any further by analysis even though it is known that the greatest number that cannot be expressed as the sum of two abundant numbers is less than this limit.
///
/// Find the sum of all the positive integers which cannot be written as the sum of two abundant numbers.
pub struct Solver023 {
    pub n: usize,
}

impl Default for Solver023 {
    fn default() -> Self {
        Self { n: 28_123 }
    }
}

impl Solver for Solver023 {
    fn problem_name(&self) -> &str { "Non-abundant sums" }

    fn solve(&self) -> i64 {
        let (mut abundant, mut list) = (vec![false; self.n + 1], Vec::with_capacity(self.n));
        arithmetic_sum(self.n.as_i64()) - (1..=self.n).filter(|&i| {
            // abundant numbers are even or multiples of 5. the boolean array is faster than a bit_set() as long is the size is known upfront.
            if (i % 2 == 0 || i % 5 == 0) && is_abundant_usize(&i) {
                abundant[i] |= true;
                list.push(i);
            }
            list.iter().take_while(|&&j| j <= i >> 1).any(|&j| abundant[i - j])
        }).sum::<usize>().as_i64()
    }
}
