// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::digits::digits_sum;
use algorithm::root::pow_10;
use Solver;

const CELL_THRESHOLD: u64 = pow_10(12);

/// `n!` means `n × (n − 1) × ... × 3 × 2 × 1`
///
/// For example, `10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800`, and the sum of the digits in the number `10!` is `3 + 6 + 2 + 8 + 8 + 0 + 0 = 27`.
///
/// Find the sum of the digits in the number `100!`
pub struct Solver020 {
    pub n: u64,
}

impl Default for Solver020 {
    fn default() -> Self {
        Self { n: 100 }
    }
}

impl Solver for Solver020 {
    fn problem_name(&self) -> &str { "Factorial digit sum" }

    fn solve(&self) -> i64 {
        let mut factorial = vec![1];
        (2..=self.n).for_each(|n| {
            let mut carry = None;
            for cell in &mut factorial {
                let value = *cell * n + carry.unwrap_or_default();

                // adjust the buckets that grow beyond the ceiling value, carrying to next bucket
                (carry, *cell) = if value > CELL_THRESHOLD { (Some(value / CELL_THRESHOLD), value % CELL_THRESHOLD) } else { (None, value) }
            }
            carry.iter().for_each(|&c| factorial.push(c));
        });
        factorial.into_iter().map(digits_sum).sum()
    }
}
