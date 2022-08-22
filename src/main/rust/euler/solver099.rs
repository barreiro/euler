// COPYRIGHT (C) 2022 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::fs::read_to_string;
use std::path::Path;

use euler::Solver;

// Comparing two numbers written in index form like 2^11 and 3^7 is not difficult, as any calculator would confirm that 2^11 = 2048 < 3^7 = 2187.
//
// However, confirming that 632382^518061 > 519432^525806 would be much more difficult, as both numbers contain over three million digits.
//
// Using base_exp.txt (right click and 'Save Link/Target As...'), a 22K text file containing one thousand lines with a base/exponent pair on each line, determine which line number has the greatest numerical value.
//
// NOTE: The first two lines in the file represent the numbers in the example given above.

const INPUT_FILE: &str = "src/main/resources/net/projecteuler/barreiro/problem/problem099-data.txt";

pub struct Solver099 {
    pub n: isize,
    pub input: String,
}

impl Default for Solver099 {
    fn default() -> Self {
        Solver099 { n: 1000, input: read_to_string(Path::new(INPUT_FILE)).expect("Unable to read file") }
    }
}

impl Solver for Solver099 {
    fn solve(&self) -> isize {
        // let pairs = self.input.lines().take(self.n as _).filter_map(|line| line.split_once(',')).map(|(base, exp)| (base.parse::<isize>().unwrap(), exp.parse::<isize>().unwrap())).collect::<Vec<_>>();
        // (1..=pairs.len()).max_by_key(|i| ((pairs[i-1].0 as f64).log2() * pairs[i-1].1 as f64) as isize ).unwrap() as _

        let as_tuple = |line: &str| line.split_once(',').map(|(base, exp)| (base.parse::<f64>().unwrap(), exp.parse::<f64>().unwrap()));
        self.input.lines().take(self.n as _).filter_map(as_tuple).zip(1..).max_by_key(|&((base, exp), _)| (base.log2() * exp) as usize ).unwrap().1
    }
}
