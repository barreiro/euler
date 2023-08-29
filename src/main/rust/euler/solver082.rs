// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::cmp::min;

use algorithm::cast::Cast;
use algorithm::io::{load_default_data, str_to_matrix};
use Solver;

/// NOTE: This problem is a more challenging version of *Problem 081*.
///
/// The minimal path sum in the `5` by `5` matrix below, by starting in any cell in the left column and finishing in any cell in the right column, and only moving up, down, and right, is indicated in red and bold; the sum is equal to `994`.
/// ```
/// 131 673 234 103  18
/// 201  96 342 965 150
/// 630 803 746 422 111
/// 537 699 497 121 956
/// 805 732 524  37 331
/// ```
/// Find the minimal path sum from the left column to the right column in `matrix.txt` (right click and "Save Link/Target As..."), a 31K text file containing an `80` by `80` matrix.
pub struct Solver082 {
    pub n: usize,
    pub input: String,
}

impl Default for Solver082 {
    fn default() -> Self {
        Self { n: 80, input: load_default_data(82) }
    }
}

impl Solver for Solver082 {
    fn problem_name(&self) -> &str { "Path sum: three ways" }

    fn solve(&self) -> i64 {
        let (mut matrix, last) = (str_to_matrix(&self.input, self.n), self.n - 1);

        // Dijkstra algorithm ends up being slower than just folding the matrix right to left
        // fold the matrix right to left, and adjust when there is a 'shorter' path up or down
        (0..last).rev().for_each(|a| {
            let (original, prev) = ((0..=last).map(|i| matrix[i][a]).collect::<Vec<_>>(), a + 1);
            (0..=last).for_each(|i| {
                matrix[i][a] += min(matrix[i][prev], min(
                    if i == 0 { i64::MAX } else { matrix[i - 1][a] },
                    if i == last { i64::MAX } else { matrix[i + 1][a] + matrix[i + 1][prev] }));

                if matrix[i][a] != original[i] + matrix[i][prev] { // check if any of the already computed values can be 'relaxed'
                    let up = (0..i).rev().take_while(|&j| matrix[j][prev] > matrix[j + 1][a]).collect::<Vec<_>>();
                    up.iter().for_each(|&j| matrix[j][a] = min(matrix[j][a], original[j] + matrix[j + 1][a]));
                }
            });
        });
        (0..=last).map(|i| matrix[i][0]).min().as_i64()
    }
}
