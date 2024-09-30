// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use crate::algorithm::cast::Cast;
use crate::algorithm::digits::first_digits;
use crate::algorithm::io::load_default_data;
use crate::euler::Solver;

/// Work out the first ten digits of the sum of the following one-hundred 50-digit numbers.
pub struct Solver013 {
    pub n: usize,
    pub input: Vec<String>,
}

impl Default for Solver013 {
    fn default() -> Self {
        Self { n: 10, input: load_default_data(13).lines().map(String::from).collect() }
    }
}

impl Solver for Solver013 {
    fn problem_name(&self) -> &str { "Large sum" }

    fn solve(&self) -> i64 {
        first_digits(self.input.iter().filter_map(|s| s[..=self.n].parse::<u64>().ok()).sum(), self.n).as_i64()
    }
}
