// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::long::square;
use euler::Solver;

// Starting with the number 1 and moving to the right in a clockwise direction a 5 by 5 spiral is formed as follows:
// 21 22 23 24 25
// 20  7  8  9 10
// 19  6  1  2 11
// 18  5  4  3 12
// 17 16 15 14 13
// It can be verified that the sum of the numbers on the diagonals is 101.
// What is the sum of the numbers on the diagonals in a 1001 by 1001 spiral formed in the same way?

pub struct Solver028 {
    pub n: isize,
}

impl Default for Solver028 {
    fn default() -> Self {
        Solver028 { n: 1001 }
    }
}

impl Solver for Solver028 {
    fn solve(&self) -> isize {
        // sum of the left corners == right corners == 2*i*i - 3*(i-1)
        1 + (3..=self.n).step_by(2).map(|i| (square(i) << 2) - 6 * (i - 1)).sum::<isize>()
    }
}
