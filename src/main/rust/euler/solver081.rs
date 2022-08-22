// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::cmp::min;
use std::fs::read_to_string;
use std::path::Path;

use euler::Solver;

// In the 5 by 5 matrix below, the minimal path sum from the top left to the bottom right, by only moving to the right and down, is indicated in bold red and is equal to 2427.
//
// 131 673 234 103  18
// 201  96 342 965 150
// 630 803 746 422 111
// 537 699 497 121 956
// 805 732 524  37 331
//
// Find the minimal path sum from the top left to the bottom right by only moving right and down in matrix.txt (right click and "Save Link/Target As..."), a 31K text file containing an 80 by 80 matrix.

const INPUT_FILE: &str = "src/main/resources/net/projecteuler/barreiro/problem/problem081-data.txt";

pub struct Solver081 {
    pub n: isize,
    pub input: String,
}

impl Default for Solver081 {
    fn default() -> Self {
        Solver081 { n: 80, input: read_to_string(Path::new(INPUT_FILE)).expect("Unable to read file") }
    }
}

impl Solver for Solver081 {
    fn solve(&self) -> isize {
        let (mut matrix, last) = (str_to_matrix(self), (self.n - 1) as usize);

        // Dijkstra algorithm ends up being slower than just folding the matrix right to left
        // preliminary step to fold the right and bottom borders
        (0..last).rev().for_each(|i| {
            matrix[i][last] += matrix[i + 1][last];
            matrix[last][i] += matrix[last][i + 1];
        });
        (0..last).rev().for_each(|a| (0..last).rev().for_each(|b| matrix[a][b] += min(matrix[a + 1][b], matrix[a][b + 1])));
        matrix[0][0]
    }
}

// --- //

fn str_to_matrix(solver: &Solver081) -> Vec<Vec<isize>> {
    let mut parsed = vec![];
    for line in solver.input.lines().take(solver.n as _) {
        parsed.push(line.split(',').take(solver.n as _).filter_map(|s| s.parse().ok()).collect());
    }
    parsed
}
