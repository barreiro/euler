// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::fs::read_to_string;
use std::path::Path;
use std::str::FromStr;

use euler::algorithm::long::to_digits;
use euler::Solver;

// For a number written in Roman numerals to be considered valid there are basic rules which must be followed.
// Even though the rules allow some numbers to be expressed in more than one way there is always a "best" way of writing a particular number.
//
// For example, it would appear that there are at least six ways of writing the number sixteen:
//
// IIIIIIIIIIIIIIII
// VIIIIIIIIIII
// VVIIIIII
// XIIIIII
// VVVI
// XVI
//
// However, according to the rules only XIIIIII and XVI are valid, and the last example is considered to be the most efficient, as it uses the least number of numerals.
//
// The 11K text file, roman.txt (right click and 'Save Link/Target As...'), contains one thousand numbers written in valid, but not necessarily minimal, Roman numerals; see About... Roman Numerals for the definitive rules for this problem.
// Find the number of characters saved by writing each of these in their minimal form.
//
// Note: You can assume that all the Roman numerals in the file contain no more than four consecutive identical units.

const INPUT_FILE: &str = "src/main/resources/net/projecteuler/barreiro/problem/problem089-data.txt";
const MINIMAL_FORM: &[usize] = &[0, 1, 2, 3, 2, 1, 2, 3, 4, 2];

pub struct Solver089 {
    pub n: isize,
    pub input: String,
}

impl Default for Solver089 {
    fn default() -> Self {
        Solver089 { n: 1_000, input: read_to_string(Path::new(INPUT_FILE)).expect("Unable to read file") }
    }
}

impl Solver for Solver089 {
    fn solve(&self) -> isize {
        self.input.lines().take(self.n as _).map(|s| (s.len() - s.parse::<Roman>().unwrap().minimal_len()) as isize).sum()
    }
}

// --- //

struct Roman {
    digits: Vec<usize>
}

impl FromStr for Roman {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Roman { digits: s.chars().flat_map(|c| match c {
            'I' => Some(1),
            'V' => Some(5),
            'X' => Some(10),
            'L' => Some(50),
            'C' => Some(100),
            'D' => Some(500),
            'M' => Some(1000),
            _ => None
        }).collect() })
    }
}

impl Roman {
    fn minimal_len(&self) -> usize {
        // conversion from roman to decimal. sum all digits and then subtract twice the 'out of order'
        let mut value = self.digits.iter().sum::<usize>();
        for i in 0..self.digits.len() - 1 {
            if self.digits[i] < self.digits[i + 1] {
                value -= self.digits[i] << 1;
            }
        }
        value / 1000 + to_digits(value as isize % 1000).iter().map(|&d| MINIMAL_FORM[d as usize]).sum::<usize>()
    }
}
