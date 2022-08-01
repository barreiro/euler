// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::collections::HashMap;

use euler::algorithm::long::{cube, from_digits, pow_10, to_digits};
use euler::Solver;

// The cube, 41063625 (345^3), can be permuted to produce two other cubes: 56623104 (384^3) and 66430125 (405^3).
// In fact, 41063625 is the smallest cube which has exactly three permutations of its digits which are also cube.
//
// Find the smallest cube for which exactly five permutations of its digits are cube.

pub struct Solver062 {
    pub n: isize
}

impl Default for Solver062 {
    fn default() -> Self {
        Solver062 { n: 5 }
    }
}

impl Solver for Solver062 {
    fn solve(&self) -> isize {
        let (floor, mut map) = (pow_10(self.n), HashMap::new());
        let hash = |i| {
            let mut digits = to_digits(i);
            digits.sort_unstable();
            from_digits(digits)
        };

        (1..).map(cube).skip_while(|&cube| cube < floor).find_map(|cube| {
            let cubes = map.entry(hash(cube)).or_insert_with(Vec::new);
            cubes.push(cube);
            Some(cubes[0]).filter(|_| cubes.len() == self.n as usize)
        }).unwrap()
    }
}
