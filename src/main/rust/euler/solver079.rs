// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;

use algorithm::cast::char_as_i64;
use algorithm::digits::{Digit, from_raw_digits};
use algorithm::io::load_default_data;
use euler::algorithm::cast::Cast;
use euler::Solver;

/// A common security method used for online banking is to ask the user for three random characters from a passcode.
/// For example, if the passcode was `531278`, they may ask for the `2nd`, `3rd`, and `5th` characters; the expected reply would be: `317`.
///
/// The text file, `keylog.txt`, contains fifty successful login attempts.
///
/// Given that the three characters are always asked for in order, analyse the file so as to determine the shortest possible secret passcode of unknown length.
pub struct Solver079 {
    pub n: usize,
    pub input: String,
}

impl Default for Solver079 {
    fn default() -> Self {
        Self { n: 50, input: load_default_data(79) }
    }
}

impl Solver for Solver079 {
    fn solve(&self) -> i64 {
        let (mut before, to_chars) = (HashMap::new(), |s: &str| s.chars().map(char_as_i64).filter_map(|i| Digit::try_from(i).ok()).collect::<Vec<_>>());
        self.input.lines().take(self.n).map(to_chars).for_each(|v| {
            (0..v.len()).for_each(|i| {
                let entry = before.entry(v[i]).or_insert_with(HashSet::new);
                (0..i).for_each(|j| { entry.insert(v[j]); });
            });
        });
        let mut guess = before.keys().copied().collect::<Vec<_>>();
        guess.sort_unstable_by_key(|g| before.len() - before.get(g).expect("Should be an entry for every character").len());
        from_raw_digits(&guess).as_i64()
    }
}

