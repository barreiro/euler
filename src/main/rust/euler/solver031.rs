// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::cast::Cast;
use algorithm::combinatorics::partition_with_constrains;
use Solver;

const DEFAULT_CURRENCY: &[u64] = &[1, 2, 5, 10, 20, 50, 100, 200];

/// In England the currency is made up of pound, `£`, and pence, `p`, and there are eight coins in general circulation: `1p, 2p, 5p, 10p, 20p, 50p, £1 (100p)` and `£2 (200p)`.
///
/// It is possible to make `£2` in the following way: `1×£1 + 1×50p + 2×20p + 1×5p + 1×2p + 3×1p`
///
/// How many different ways can `£2` be made using any number of coins?
pub struct Solver031 {
    pub n: u64,
    pub currency: Vec<u64>,
}

impl Default for Solver031 {
    fn default() -> Self {
        Self { n: 200, currency: DEFAULT_CURRENCY.to_vec() }
    }
}

impl Solver for Solver031 {
    fn problem_name(&self) -> &str { "Coin sums" }

    fn solve(&self) -> i64 {
        partition_with_constrains(self.n, &self.currency).as_i64()
    }
}
