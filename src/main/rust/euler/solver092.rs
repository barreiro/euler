// COPYRIGHT (C) 2022 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::cast::{Cast, UCast};
use algorithm::digits::digits_square_sum;
use algorithm::root::{square, square_u64};
use Solver;

// numbers that converge to 1 are known as "happy numbers"
const HAPPY: u64 = 1;
const UNHAPPY: u64 = 89;

/// A number chain is created by continuously adding the square of the digits in a number to form a new number until it has been seen before.
///
/// For example:
///      `44 → 32 → 13 → 10 → 1 → 1`
///      `85 → 89 → 145 → 42 → 20 → 4 → 16 → 37 → 58 → 89`
///
/// Therefore any chain that arrives at `1` or `89` will become stuck in an endless loop. What is most amazing is that *EVERY* starting number will eventually arrive at `1` or `89`.
/// How many starting numbers below ten million will arrive at `89`?
pub struct Solver092 {
    pub n: u64,
}

impl Default for Solver092 {
    fn default() -> Self {
        Self { n: 7 }
    }
}

impl Solver for Solver092 {
    fn solve(&self) -> i64 {
        let (ceil, is_happy) = (square_u64(9) * self.n, |mut n| loop {
            match n {
                HAPPY => break true,
                UNHAPPY => break false,
                _ => n = digits_square_sum(n)
            }
        });
        let mut f_cache = (0..=ceil).map(|_| (0..=self.n).map(|_| None).collect()).collect();

        // find the sums that can lead to UNHAPPY. loop and them use the f(n,k) function to calculate the number of digit combinations that yield that sum
        (2..=ceil).filter(|&n| !is_happy(n)).map(|n| f(n.as_usize(), self.n.as_usize(), &mut f_cache)).sum::<u64>().as_i64()
    }
}

// let the number of ways of writing n as the sum of k squares be f(n, k)
// f(n, k) = f(n-0^2, k-1) + f(n-1^2, k-1) + f(n-2^2, k-1) + ... + f(n-9^2, k-1)
// with f(n, k) = 0 if n < 0; f(n, 0) = 0 if n > 0; f(0, 0) = 1
fn f(n: usize, k: usize, cache: &mut Vec<Vec<Option<u64>>>) -> u64 {
    if cache[n][k].is_none() {
        cache[n][k] = Some(
            if k == 0 {
                u64::from(n == 0)
            } else {
                (0..=9).map(|d| n.as_i64() - square(d)).filter(|&nd| nd >= 0).map(|nd| f(nd.as_u64().as_usize(), k - 1, cache)).sum()
            }
        );
    }
    cache[n][k].unwrap()
}
