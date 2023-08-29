// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::cmp::min;

use algorithm::io::{load_default_data, str_to_matrix};
use Solver;

/// In the `5` by `5` matrix below, the minimal path sum from the top left to the bottom right, by only moving to the right and down, is indicated in bold red and is equal to `2427`.
/// ```
/// 131 673 234 103  18
/// 201  96 342 965 150
/// 630 803 746 422 111
/// 537 699 497 121 956
/// 805 732 524  37 331
/// ```
/// Find the minimal path sum from the top left to the bottom right by only moving right and down in `matrix.txt` (right click and "Save Link/Target As..."), a 31K text file containing an `80` by `80` matrix.
pub struct Solver081 {
    pub n: usize,
    pub input: String,
}

impl Default for Solver081 {
    fn default() -> Self {
        Self { n: 80, input: load_default_data(81) }
    }
}

impl Solver for Solver081 {
    fn problem_name(&self) -> &str { "Path sum: two ways" }

    fn solve(&self) -> i64 {
        let (mut matrix, last) = (str_to_matrix(&self.input, self.n), self.n - 1);

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
