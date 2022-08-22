// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::fs::read_to_string;
use std::path::Path;

use euler::algorithm::long::is_triangle;
use euler::Solver;

// The n-th term of the sequence of triangle numbers is given by, t(n) = 1/2 n (n+1); so the first ten triangle numbers are:
// 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
//
// By converting each letter in a word to a number corresponding to its alphabetical position and adding these values we form a word value.
// For example, the word value for SKY is 19 + 11 + 25 = 55 = t(10).
//
// If the word value is a triangle number then we shall call the word a triangle word.
// Using words.txt (right click and 'Save Link/Target As...'), a 16K text file containing nearly two-thousand common English words, how many are triangle words?

const INPUT_FILE: &str = "src/main/resources/net/projecteuler/barreiro/problem/problem042-data.txt";

pub struct Solver042 {
    pub n: isize,
    pub input: String,
}

impl Default for Solver042 {
    fn default() -> Self {
        Solver042 { n: 1786, input: read_to_string(Path::new(INPUT_FILE)).expect("Unable to read file") }
    }
}

impl Solver for Solver042 {
    fn solve(&self) -> isize {
        let char_sum = |name: &str| name.chars().map(|c| 1 + c as isize - 'A' as isize).sum::<_>();
        self.input.split(',').map(|s| s.trim_matches('\"')).take(self.n as _).map(char_sum).filter(|&sum| is_triangle(sum)).count() as _
    }
}
