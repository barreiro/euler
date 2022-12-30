// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::cast::Cast;
use algorithm::digits::first_digits;
use algorithm::io::load_default_data;
use euler::Solver;

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
    fn solve(&self) -> i64 {
        first_digits(self.input.iter().map(|s| s[..=self.n as usize].parse::<u64>().unwrap()).sum(), self.n).as_i64()
    }
}
