// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::collections::HashMap;

use euler::algorithm::factor::factor_composition;
use euler::algorithm::prime::prime_factors;
use euler::Solver;

// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

pub struct Solver005 {
    pub n: isize
}

impl Default for Solver005 {
    fn default() -> Self {
        Solver005 { n: 20 }
    }
}

impl Solver for Solver005 {
    fn solve(&self) -> isize {
        let mut factors_of_smallest = HashMap::new();
        for l in 2..self.n {
            for (key, value) in prime_factors(l) {
                *factors_of_smallest.entry(key).or_insert(0) = value.max(*factors_of_smallest.get(&key).unwrap_or(&0));
            }
        }
        factor_composition(factors_of_smallest)
    }
}
