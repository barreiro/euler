// COPYRIGHT (C) 2022 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use crate::algorithm::cast::Cast;
use crate::algorithm::io::load_default_data;
use crate::Solver;

/// Comparing two numbers written in index form like `2^11` and `3^7` is not difficult, as any calculator would confirm that `2^11 = 2048 < 3^7 = 2187`.
///
/// However, confirming that `632382^518061 > 519432^525806` would be much more difficult, as both numbers contain over three million digits.
///
/// Using `base_exp.txt` (right click and 'Save Link/Target As...'), a 22K text file containing one thousand lines with a base/exponent pair on each line, determine which line number has the greatest numerical value.
///
/// NOTE: The first two lines in the file represent the numbers in the example given above.
pub struct Solver099 {
    pub n: usize,
    pub input: String,
}

impl Default for Solver099 {
    fn default() -> Self {
        Self { n: 1000, input: load_default_data(99) }
    }
}

impl Solver for Solver099 {
    fn problem_name(&self) -> &str { "Largest exponential" }

    fn solve(&self) -> i64 {
        let as_tuple = |line: &str| line.split_once(',').map(|(base, exp)| (base.parse::<f64>().expect("Base should be a well formatted number"), exp.parse::<f64>().expect("Exp should be a well formatted number")));
        (1..).zip(self.input.lines().take(self.n).filter_map(as_tuple)).max_by_key(|&(_, (base, exp))| (base.log2() * exp).as_i64()).expect("Input should not be empty").0
    }
}
