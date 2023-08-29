// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::iter::from_fn;

use algorithm::cast::Cast;
use algorithm::digits::digits_sum;
use algorithm::root::pow_10;
use Solver;

/// A googol `(10^100)` is a massive number: one followed by one-hundred zeros; `100^100` is almost unimaginably large: one followed by two-hundred zeros.
/// Despite their size, the sum of the digits in each number is only `1`.
///
/// Considering natural numbers of the form, `a^b`, where `a, b < 100`, what is the maximum digital sum?
pub struct Solver056 {
    pub n: usize,
}

impl Default for Solver056 {
    fn default() -> Self {
        Self { n: 100 }
    }
}

impl Solver for Solver056 {
    fn problem_name(&self) -> &str { "Powerful digit sum" }

    fn solve(&self) -> i64 {
        // only test a fraction of the space, just the 10% biggest numbers!
        let (floor, ceil) = (9 * self.n / 10, self.n);
        (floor..ceil).filter_map(|a| vectorized_power(a.as_u64()).skip(floor - 1).take(ceil - floor).map(|power| power.into_iter().map(digits_sum).sum::<i64>()).max()).max().as_i64()
    }
}

// --- //

/// calculates consecutive powers of a given base
fn vectorized_power(base: u64) -> impl Iterator<Item=Vec<u64>> {
    let (mut power, threshold) = (vec![1], pow_10(10));
    from_fn(move || {
        let mut carry = None;
        for cell in &mut power {
            let value = *cell * base + carry.unwrap_or_default();

            // adjust the buckets that grow beyond the ceiling value, carrying to next bucket
            (*cell, carry) = if value > threshold { (value % threshold, Some(value / threshold)) } else { (value, None) };
        }

        // with a small cell_threshold values would probably need to split the carry into cells as well
        carry.iter().for_each(|&c| power.push(c));
        Some(power.clone())
    })
}
