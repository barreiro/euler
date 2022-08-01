// COPYRIGHT (C) 2022 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::long::{digits_square_sum, square};
use euler::Solver;

// A number chain is created by continuously adding the square of the digits in a number to form a new number until it has been seen before.
//
// For example:
//      44 → 32 → 13 → 10 → 1 → 1
//      85 → 89 → 145 → 42 → 20 → 4 → 16 → 37 → 58 → 89
//
// Therefore any chain that arrives at 1 or 89 will become stuck in an endless loop. What is most amazing is that EVERY starting number will eventually arrive at 1 or 89.
// How many starting numbers below ten million will arrive at 89?

const HAPPY: isize = 1; // numbers that converge to 1 are known as "happy numbers"
const UNHAPPY: isize = 89;

pub struct Solver092 {
    pub n: isize,
}

impl Default for Solver092 {
    fn default() -> Self {
        Solver092 { n: 7 }
    }
}

impl Solver for Solver092 {
    fn solve(&self) -> isize {
        let (ceil, is_happy) = (square(9) * self.n, |mut n| loop {
            match n {
                HAPPY => break true,
                UNHAPPY => break false,
                _ => n = digits_square_sum(n)
            }
        });

        // let mut cache = (1..=ceil).map(|n| !is_happy(n)).collect::<Vec<_>>();
        // cache.insert(0, false);
        // (1..pow_10(self.n)).filter(|&n| cache[digits_square_sum(n) as usize]).count() as _

        // let cache = (1..=ceil).filter(|&n| !is_happy(n)).collect::<BitSet>();
        // (1..pow_10(self.n)).filter(|&n| cache.contains(digits_square_sum(n))).count() as _

        let mut f_cache = (0..=ceil).map(|_| (0..=self.n).map(|_| None).collect()).collect();

        // find the sums that can lead to UNHAPPY. loop and them use the f(n,k) function to calculate the number of digit combinations that yield that sum
        (2..=ceil).filter(|&n| !is_happy(n)).map(|n| f(n as usize, self.n as usize, &mut f_cache)).sum()
    }
}

// let the number of ways of writing n as the sum of k squares be f(n, k)
// f(n, k) = f(n-0^2, k-1) + f(n-1^2, k-1) + f(n-2^2, k-1) + ... + f(n-9^2, k-1)
// with f(n, k) = 0 if n < 0; f(n, 0) = 0 if n > 0; f(0, 0) = 1
fn f(n: usize, k: usize, cache: &mut Vec<Vec<Option<isize>>>) -> isize {
    if cache[n][k].is_none() {
        cache[n][k] = Some(
            if k == 0 {
                if n == 0 { 1 } else { 0 }
            } else {
                (0..=9).map(|d| n as isize - square(d)).filter(|&nd| nd >= 0).map(|nd| f(nd as usize, k - 1, cache)).sum()
            }
        )
    }
    cache[n][k].unwrap()
}
