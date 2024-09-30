// COPYRIGHT (C) 2023 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use crate::algorithm::cast::Cast;
use crate::algorithm::combinatorics::multi_choose;
use crate::algorithm::digits::DEFAULT_RADIX;
use crate::Solver;

/// Working from left-to-right if no digit is exceeded by the digit to its left it is called an increasing number; for example, `134468`.
///
/// Similarly, if no digit is exceeded by the digit to its right it is called a decreasing number; for example, `66420`.
///
/// We shall call a positive integer that is neither increasing nor decreasing a "bouncy" number; for example, `155349`.
///
/// As `n` increases, the proportion of bouncy numbers below `n` increases such that there are only `12951` numbers below one-million that are not bouncy and only `277032` non-bouncy numbers below `10^10`.
///
/// How many numbers below a googol (`10^100`) are not bouncy?
pub struct Solver113 {
    pub n: u64,
    pub radix: u64,
}

impl Default for Solver113 {
    fn default() -> Self {
        Self { n: 100, radix: DEFAULT_RADIX.as_u64() }
    }
}

impl Solver for Solver113 {
    fn problem_name(&self) -> &str { "Non-bouncy numbers" }
    
    #[allow(clippy::suspicious_operation_groupings)]
    fn solve(&self) -> i64 {
        // the number of increasing is given by `multi_choose(radix, n)`
        // treat the decreasing case like there was an extra digit (bigger than all others), to account for the cases where the first one is zero
        // finally, remove numbers that have all the same digits that are counted twice (both increasing and decreasing) and zeros

        (multi_choose(self.radix, self.n) + multi_choose(self.radix + 1, self.n) - (self.radix * self.n) - 2).as_i64()
    }
}
