// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::cmp::min;
use std::fs::read_to_string;
use std::path::Path;

use euler::Solver;

// NOTE: This problem is a more challenging version of Problem 81.
//
// The minimal path sum in the 5 by 5 matrix below, by starting in any cell in the left column and finishing in any cell in the right column, and only moving up, down, and right, is indicated in red and bold; the sum is equal to 994.
//
// 131 673 234 103  18
// 201  96 342 965 150
// 630 803 746 422 111
// 537 699 497 121 956
// 805 732 524  37 331
//
// Find the minimal path sum from the left column to the right column in matrix.txt (right click and "Save Link/Target As..."), a 31K text file containing an 80 by 80 matrix.

const INPUT_FILE: &str = "src/main/resources/net/projecteuler/barreiro/problem/problem082-data.txt";

pub struct Solver082 {
    pub n: isize,
    pub input: String,
}

impl Default for Solver082 {
    fn default() -> Self {
        Solver082 { n: 80, input: read_to_string(Path::new(INPUT_FILE)).expect("Unable to read file") }
    }
}

impl Solver for Solver082 {
    fn solve(&self) -> isize {
        let (mut matrix, last) = (str_to_matrix(self), (self.n - 1) as usize);

        // Dijkstra algorithm ends up being slower than just folding the matrix right to left
        // fold the matrix right to left, and adjust when there is a 'shorter' path up or down
        (0..last).rev().for_each(|a| {
            let (original, prev) = ((0..=last).map(|i| matrix[i][a]).collect::<Vec<_>>(), a + 1);
            (0..=last).for_each(|i| {
                matrix[i][a] += min(matrix[i][prev], min(
                    if i == 0 { isize::MAX } else { matrix[i - 1][a] },
                    if i == last { isize::MAX } else { matrix[i + 1][a] + matrix[i + 1][prev] }));

                if matrix[i][a] != original[i] + matrix[i][prev] { // check if any of the already computed values can be 'relaxed'
                    let up = (0..i).rev().take_while(|&j| matrix[j][prev] > matrix[j + 1][a]).collect::<Vec<_>>();
                    up.iter().for_each(|&j| matrix[j][a] = min(matrix[j][a], original[j] + matrix[j + 1][a]));
                }
            });
        });
        (0..=last).map(|i| matrix[i][0]).min().unwrap()
    }
}

// --- //

fn str_to_matrix(solver: &Solver082) -> Vec<Vec<isize>> {
    let mut parsed = vec![];
    for line in solver.input.lines().take(solver.n as _) {
        parsed.push(line.split(',').take(solver.n as _).filter_map(|s| s.parse().ok()).collect());
    }
    parsed
}
