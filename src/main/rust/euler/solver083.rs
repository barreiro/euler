// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::cmp::min;

use algorithm::io::{load_default_data, str_to_matrix};
use Solver;

/// In the `5` by `5` matrix below, the minimal path sum from the top left to the bottom right, by moving left, right, up, and down, is indicated in bold red and is equal to `2297`.
/// ```
/// 131 673 234 103  18
/// 201  96 342 965 150
/// 630 803 746 422 111
/// 537 699 497 121 956
/// 805 732 524  37 331
/// ```
/// Find the minimal path sum from the top left to the bottom right by moving left, right, up, and down in `matrix.txt` (right click and "Save Link/Target As..."), a 31K text file containing an `80` by `80` matrix.
pub struct Solver083 {
    pub n: usize,
    pub input: String,
}

impl Default for Solver083 {
    fn default() -> Self {
        Self { n: 80, input: load_default_data(83) }
    }
}

impl Solver for Solver083 {
    fn problem_name(&self) -> &str { "Path sum: four ways" }

    #[allow(clippy::iter_with_drain)]
    fn solve(&self) -> i64 {
        let (matrix, last) = (str_to_matrix(&self.input, self.n), self.n - 1);
        let (mut sum, mut changes, mut modified) = (matrix.clone(), Vec::with_capacity(last * last), (0..=last).flat_map(|a| (0..=last).map(move |b| (a, b))).collect::<Vec<_>>());

        // Dijkstra algorithm ends up being slower than just folding the matrix from bottom right to top left
        // preliminary step to fold the top and left borders
        (1..=last).for_each(|i| {
            sum[i][0] += sum[i - 1][0];
            sum[0][i] += sum[0][i - 1];
        });
        (1..=last).for_each(|a| (1..=last).for_each(|b| sum[a][b] += min(sum[a - 1][b], sum[a][b - 1])));

        // iterate over the sum matrix, finding better paths that go up or left.
        while modified.last() == Some(&(last, last)) {
            for (a, b) in modified {
                let candidate = min_neighbour(a, b, last, &sum) + matrix[a][b];
                if sum[a][b] > candidate {
                    sum[a][b] = candidate;
                    changes.push((a, b));
                }
            }
            modified = changes.drain(..).collect();
        }
        sum[last][last]
    }
}

fn min_neighbour(a: usize, b: usize, last: usize, m: &[Vec<i64>]) -> i64 {
    let (up, down) = (if a == 0 { i64::MAX } else { m[a - 1][b] }, if a == last { i64::MAX } else { m[a + 1][b] });
    let (left, right) = (if b == 0 { i64::MAX } else { m[a][b - 1] }, if b == last { i64::MAX } else { m[a][b + 1] });
    let (min_h, min_v) = (min(left, right), min(up, down));
    if up < min(down, min_h) {
        m[a - 1][b]
    } else if down < min(up, min_h) {
        m[a + 1][b]
    } else if right < min(left, min_v) {
        m[a][b + 1]
    } else {
        m[a][b - 1]
    }
}
