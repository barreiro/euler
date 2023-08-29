// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::cast::Cast;
use algorithm::long::arithmetic_sum;
use algorithm::root::int_sqrt;
use Solver;

/// By counting carefully it can be seen that a rectangular grid measuring `3` by `2` contains eighteen rectangles.
///
/// Although there exists no rectangular grid that contains exactly two million rectangles, find the area of the grid with the nearest solution.
pub struct Solver085 {
    pub n: i64,
}

impl Default for Solver085 {
    fn default() -> Self {
        Self { n: 2_000_000 }
    }
}

impl Solver for Solver085 {
    fn problem_name(&self) -> &str { "Counting rectangles" }

    fn solve(&self) -> i64 {
        // number of rectangles in a n * m grid = arithmetic_sum(n) * arithmetic_sum(m)
        (1..=int_sqrt(int_sqrt(self.n * 4))).map(|n| {
            let (m, diff) = (int_sqrt((self.n * 2) / arithmetic_sum(n)), |y| (self.n - arithmetic_sum(n) * arithmetic_sum(y)).abs());
            if diff(m) < diff(m - 1) { (n, m, diff(m)) } else { (n, m - 1, diff(m - 1)) }
        }).min_by_key(|&(_, _, r)| r).map(|(x, y, _)| x * y).as_i64()
    }
}
