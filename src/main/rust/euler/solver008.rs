// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::cast::map_char_as_i64;
use algorithm::io::load_default_data;
use algorithm::vec::array_product;
use Solver;

/// The four adjacent digits in the `1000-digit` number that have the greatest product are `9 × 9 × 8 × 9 = 5832`.
/// Find the thirteen adjacent digits in the `1000-digit` number that have the greatest product. What is the value of this product?
pub struct Solver008 {
    pub n: usize,
    pub input: String,
}

impl Default for Solver008 {
    fn default() -> Self {
        Self { n: 13, input: load_default_data(8) }
    }
}

impl Solver for Solver008 {
    fn solve(&self) -> i64 {
        let digits: Vec<i64> = self.input.chars().map(map_char_as_i64).collect();
        digits.windows(self.n).map(array_product).max().unwrap()
    }
}
