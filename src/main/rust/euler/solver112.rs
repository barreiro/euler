// COPYRIGHT (C) 2023 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::cast::Cast;
use algorithm::digits::incrementing_digits;
use algorithm::long::GetAndIncrement;
use Solver;

/// Working from left-to-right if no digit is exceeded by the digit to its left it is called an increasing number; for example, `134468`.
///
/// Similarly if no digit is exceeded by the digit to its right it is called a decreasing number; for example, `66420`.
///
/// We shall call a positive integer that is neither increasing nor decreasing a "bouncy" number; for example, `155349`.
///
/// Clearly there cannot be any bouncy numbers below one-hundred, but just over half of the numbers below one-thousand (`525`) are bouncy.
/// In fact, the least number for which the proportion of bouncy numbers first reaches `50%` is `538`.
///
/// Surprisingly, bouncy numbers become more and more common and by the time we reach `21780` the proportion of bouncy numbers is equal to `90%`.
///
/// Find the least number for which the proportion of bouncy numbers is exactly `99%`.
pub struct Solver112 {
    pub n: usize,
}

impl Default for Solver112 {
    fn default() -> Self {
        Self { n: 99 }
    }
}

impl Solver for Solver112 {
    fn solve(&self) -> i64 {
        incrementing_digits().scan((0, 0), |(total, bouncy), digits| {
            (!digits.is_increasing() && !digits.is_decreasing()).then(|| bouncy.get_and_increment());
            (total.get_and_increment() * self.n >= *bouncy * 100).then_some(digits)
        }).last().as_i64()
    }
}
