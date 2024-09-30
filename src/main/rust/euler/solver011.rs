// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use crate::algorithm::io::load_default_data;
use crate::Solver;

/// In the 20×20 grid below, four numbers along a diagonal line have been marked in red.
///
/// The product of these numbers is `26 × 63 × 78 × 14 = 1788696`.
///
/// What is the greatest product of four adjacent numbers in the same direction (up, down, left, right, or diagonally) in the 20×20 grid?
pub struct Solver011 {
    pub n: usize,
    pub input: String,
}

impl Default for Solver011 {
    fn default() -> Self {
        Self { n: 4, input: load_default_data(11) }
    }
}

impl Solver for Solver011 {
    fn problem_name(&self) -> &str { "Largest product in a grid" }

    fn solve(&self) -> i64 {
        let (mut greatest, grid) = (0, grid_str(&self.input));
        for n in self.n - 1..grid.len() {
            for m in self.n - 1..grid[n].len() {
                let (mut row, mut column, mut diag_a, mut diag_b) = (1, 1, 1, 1);
                for i in 0..self.n {
                    row *= grid[n][m - i];
                    column *= grid[n - i][m];
                    diag_a *= grid[n - i][m - i];
                    diag_b *= grid[n - i][m + i + 1 - self.n];
                }
                greatest = greatest.max(row.max(column.max(diag_a.max(diag_b))));
            }
        }
        greatest
    }
}

// --- //

fn grid_str(grid_str: &str) -> Vec<Vec<i64>> {
    grid_str.lines().map(|line|
        line.split_whitespace().map(|cell|
            cell.parse().unwrap_or_default()
        ).collect()
    ).collect()
}
