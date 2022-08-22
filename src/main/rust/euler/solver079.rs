// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::path::Path;

use euler::algorithm::cast::Cast;
use euler::algorithm::long::from_digits;
use euler::Solver;

// A common security method used for online banking is to ask the user for three random characters from a passcode.
// For example, if the passcode was 531278, they may ask for the 2nd, 3rd, and 5th characters; the expected reply would be: 317.
//
// The text file, keylog.txt, contains fifty successful login attempts.
//
// Given that the three characters are always asked for in order, analyse the file so as to determine the shortest possible secret passcode of unknown length.

const INPUT_FILE: &str = "src/main/resources/net/projecteuler/barreiro/problem/problem079-data.txt";

pub struct Solver079 {
    pub n: isize,
    pub input: String,
}

impl Default for Solver079 {
    fn default() -> Self {
        Solver079 { n: 50, input: read_to_string(Path::new(INPUT_FILE)).expect("Unable to read file") }
    }
}

impl Solver for Solver079 {
    fn solve(&self) -> isize {
        let (mut before, to_chars) = (HashMap::new(), |s: &str| s.chars().map(Cast::isize).collect::<Vec<_>>());
        self.input.lines().take(self.n as _).map(to_chars).for_each(|v| {
            (0..v.len()).for_each(|i| {
                let entry = before.entry(v[i]).or_insert_with(HashSet::new);
                (0..i).for_each(|j| { entry.insert(v[j]); });
            });
        });
        let mut guess = before.keys().copied().collect::<Vec<_>>();
        guess.sort_unstable_by_key(|g| before.len() - before.get(g).unwrap().len());
        from_digits(guess)
    }
}

