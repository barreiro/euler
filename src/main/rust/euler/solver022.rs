// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::fs;
use std::path::Path;

use euler::Solver;

// Using names.txt (right click and 'Save Link/Target As...'), a 46K text file containing over five-thousand first names, begin by sorting it into alphabetical order. Then working out the alphabetical value for each name, multiply this value by its alphabetical position in the list to obtain a name score.
// For example, when the list is sorted into alphabetical order, COLIN, which is worth 3 + 15 + 12 + 9 + 14 = 53, is the 938th name in the list. So, COLIN would obtain a score of 938 × 53 = 49714.
// What is the total of all the name scores in the file?

const BASE_PATH: &str = "src/main/resources/net/projecteuler/barreiro/problem/";

pub struct Solver022 {
    pub n: isize,
    pub input: String,
}

impl Default for Solver022 {
    fn default() -> Self {
        let location = BASE_PATH.to_string() + "problem022-data.txt";
        let path = Path::new(location.trim());
        Solver022 { n: 5163, input: fs::read_to_string(path).expect("Unable to read file") }
    }
}

impl Solver for Solver022 {
    fn solve(&self) -> isize {
        let mut names = self.input.split(',').map(|s| s.trim_matches('\"')).collect::<Vec<_>>();
        names.sort();
        names.iter().take(self.n as _).enumerate().map(|(i, name)| (i as isize + 1) * name.chars().map(|c| 1 + c as isize - 'A' as isize).sum::<isize>()).sum()
    }
}
