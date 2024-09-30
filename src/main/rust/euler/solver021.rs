// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use crate::algorithm::cast::Cast;
use crate::algorithm::factor::sum_of_factors;
use crate::Solver;

/// Let `d(n)` be defined as the sum of proper divisors of `n` (numbers less than `n` which divide evenly into `n`).
///
/// If `d(a) = b` and `d(b) = a`, where `a ≠ b`, then `a` and `b` are an amicable pair and each of `a` and `b` are called amicable numbers.
///
/// For example, the proper divisors of `220` are `1, 2, 4, 5, 10, 11, 20, 22, 44, 55` and `110`; therefore `d(220) = 284`.
///
/// The proper divisors of `284` are `1, 2, 4, 71` and `142`; so `d(284) = 220`.
///
/// Evaluate the sum of all the amicable numbers under `10000`.
pub struct Solver021 {
    pub n: i64,
}

impl Default for Solver021 {
    fn default() -> Self {
        Self { n: 10000 }
    }
}

impl Solver for Solver021 {
    fn problem_name(&self) -> &str { "Amicable numbers" }

    fn solve(&self) -> i64 {
        let mut factor_sum = Vec::with_capacity(self.n.as_usize());
        (0..self.n).filter_map(|i| {
            let sum = sum_of_factors(i);
            factor_sum.push(sum);
            Some(sum + i).filter(|_| sum < i && factor_sum[sum.as_usize()] == i)
        }).sum()
    }
}
