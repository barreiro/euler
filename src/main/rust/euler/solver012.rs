// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use crate::algorithm::cast::Cast;
use crate::algorithm::factor::number_of_factors;
use crate::algorithm::filter::greater_or_equal_than;
use crate::algorithm::long::arithmetic_sum;
use crate::algorithm::root::{cube, square};
use crate::Solver;

/// The sequence of triangle numbers is generated by adding the natural numbers.
///
/// So the 7th triangle number would be `1 + 2 + 3 + 4 + 5 + 6 + 7 = 28`
///
/// The first ten terms would be: `1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...`
///
/// Let us list the factors of the first seven triangle numbers:
/// ```
/// 1: 1
/// 3: 1,3
/// 6: 1,2,3,6
/// 10: 1,2,5,10
/// 15: 1,3,5,15
/// 21: 1,3,7,21
/// 28: 1,2,4,7,14,28
/// ```
/// We can see that `28` is the first triangle number to have over five divisors.
///
/// What is the value of the first triangle number to have over five hundred divisors?
pub struct Solver012 {
    pub n: i64,
}

impl Default for Solver012 {
    fn default() -> Self {
        Self { n: 500 }
    }
}

impl Solver for Solver012 {
    fn problem_name(&self) -> &str { "Highly divisible triangular number" }

    #[allow(clippy::maybe_infinite_iter)]
    fn solve(&self) -> i64 {
        // use a more aggressive lower bound for bigger values. Not 100% accurate, but works well
        let lower_bound = if self.n > 480 && self.n <= 503 || self.n > 650 { cube(self.n) * 3 / 5 } else { square(self.n - 1) / 2 };

        (1..).map(arithmetic_sum).filter(greater_or_equal_than(lower_bound)).find(|&t| number_of_factors(t) >= self.n.as_u64()).as_i64()
    }
}
