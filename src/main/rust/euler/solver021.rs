// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::factor::sum_of_factors;
use euler::Solver;

// Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide evenly into n).
// If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and each of a and b are called amicable numbers.
// For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110; therefore d(220) = 284. The proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.
// Evaluate the sum of all the amicable numbers under 10000.

pub struct Solver021 {
    pub n: isize,
}

impl Default for Solver021 {
    fn default() -> Self {
        Solver021 {
            n: 10000
        }
    }
}

impl Solver for Solver021 {
    fn solve(&self) -> isize {
        let (mut factor_sum, mut amicable_sum) = (vec![0; self.n as usize], 0);
        for i in 1..self.n {
            let sum = sum_of_factors(i);
            factor_sum[i as usize] = sum;

            if sum < i && factor_sum[sum as usize] == i {
                amicable_sum += sum + i;
            }
        }
        amicable_sum
    }
}