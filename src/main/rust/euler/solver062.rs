// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::collections::HashMap;

use algorithm::cast::Cast;
use algorithm::digits::Digits;
use algorithm::root::{cube_u64, pow_10};
use Solver;

/// The cube, `41063625 (345^3)`, can be permuted to produce two other cubes: `56623104 (384^3)` and `66430125 (405^3)`.
/// In fact, `41063625` is the smallest cube which has exactly three permutations of its digits which are also cube.
///
/// Find the smallest cube for which exactly five permutations of its digits are cube.
pub struct Solver062 {
    pub n: usize,
}

impl Default for Solver062 {
    fn default() -> Self {
        Self { n: 5 }
    }
}

impl Solver for Solver062 {
    fn problem_name(&self) -> &str { "Cubic permutations" }

    fn solve(&self) -> i64 {
        let (floor, mut map) = (pow_10(self.n as u64), HashMap::new());

        (1..).map(cube_u64).skip_while(|&cube| cube < floor).find_map(|cube| {
            let cubes = map.entry(Digits::from(cube).to_fingerprint()).or_insert_with(Vec::new);
            cubes.push(cube);
            (cubes.len() == self.n).then_some(cubes[0])
        }).as_i64()
    }
}
