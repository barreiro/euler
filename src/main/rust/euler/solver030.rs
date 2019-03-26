// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::Solver;
use euler::algorithm::long::pow;
use euler::algorithm::long::to_digits;

//Surprisingly there are only three numbers that can be written as the sum of fourth powers of their digits:
//1634 = 1^4 + 6^4 + 3^4 + 4^4
//8208 = 8^4 + 2^4 + 0^4 + 8^4
//9474 = 9^4 + 4^4 + 7^4 + 4^4
//As 1 = 1^4 is not a sum it is not included.
//
//The sum of these numbers is 1634 + 8208 + 9474 = 19316.
//Find the sum of all the numbers that can be written as the sum of fifth powers of their digits.

pub struct Solver030 {
    pub n: isize,
}

impl Default for Solver030 {
    fn default() -> Self {
        Solver030 {
            n: 5,
        }
    }
}

impl Solver for Solver030 {
    fn solve(&self) -> isize {
        let (mut sum, lower, upper) = (0, pow(9, self.n/2), self.n * pow(9, self.n));
        for n in lower..upper {
            let mut digit_sum = 0;
            for digit in to_digits(n) {
                digit_sum += pow(digit, self.n);
            }
            if digit_sum == n {
                sum += n;
            }
        }
        sum
    }
}
