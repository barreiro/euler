// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use crate::algorithm::cast::char_as_i64;
use crate::algorithm::io::load_default_data;

use crate::euler::Solver;

/// Using `names.txt` (right click and 'Save Link/Target As...'), a 46K text file containing over five-thousand first names, begin by sorting it into alphabetical order.
///
/// Then working out the alphabetical value for each name, multiply this value by its alphabetical position in the list to obtain a name score.
///
/// For example, when the list is sorted into alphabetical order, `COLIN`, which is worth `3 + 15 + 12 + 9 + 14 = 53`, is the `938th` name in the list. So, `COLIN` would obtain a score of `938 × 53 = 49714`.
///
/// What is the total of all the name scores in the file?
pub struct Solver022 {
    pub n: usize,
    pub input: String,
}

impl Default for Solver022 {
    fn default() -> Self {
        Self { n: 5163, input: load_default_data(22) }
    }
}

impl Solver for Solver022 {
    fn problem_name(&self) -> &str { "Names scores" }

    fn solve(&self) -> i64 {
        let mut names = self.input.split(',').map(|s| s.trim_matches('\"')).collect::<Vec<_>>();
        names.sort_unstable();
        names.iter().take(self.n).zip(1..).map(|(name, n)| n * name.chars().map(char_as_i64).sum::<i64>()).sum()
    }
}
