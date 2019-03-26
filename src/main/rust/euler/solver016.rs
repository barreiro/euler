// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::long::DEFAULT_RADIX;
use euler::Solver;

// 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
// What is the sum of the digits of the number 2^1000?

pub struct Solver016 {
    pub n: isize
}

impl Default for Solver016 {
    fn default() -> Self {
        Solver016 { n: 1000 }
    }
}

impl Solver for Solver016 {
    fn solve(&self) -> isize {
        // each element is a digit. Each iteration we double every digit and adjust
        let mut values = vec![1];

        for _ in 0..self.n {
            // since carry never ripples we can iterate backwards, using less memory
            for j in (0..values.len()).rev() {
                values[j] *= 2;
                if values[j] >= DEFAULT_RADIX {
                    // with radix > 2 can use increment and subtract instead of divide and take the remainder
                    values[j] = values[j] - DEFAULT_RADIX;
                    if j == values.len() - 1 { values.push(1) } else { values[j + 1] += 1 }
                }
            }
        }
        values.iter().sum()
    }
}
