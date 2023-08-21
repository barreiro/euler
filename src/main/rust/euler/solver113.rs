// COPYRIGHT (C) 2023 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::cast::Cast;
use algorithm::combinatorics::multi_choose;
use algorithm::digits::DEFAULT_RADIX;
use Solver;

/// Working from left-to-right if no digit is exceeded by the digit to its left it is called an increasing number; for example, `134468`.
///
/// Similarly if no digit is exceeded by the digit to its right it is called a decreasing number; for example, `66420`.
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
    fn solve(&self) -> i64 {
        // the number of increasing is given by `multi_choose(radix, n)`
        // treat the decreasing case like there was an extra digit (bigger than all others), to account for the cases where the first one is zero
        // finally, remove zeros and numbers that have all the same digits that are counted twice (both increasing ans decreasing)
        (multi_choose(self.radix, self.n) + multi_choose(self.radix + 1, self.n) - (self.n * self.radix) - 2).as_i64()
    }
}
