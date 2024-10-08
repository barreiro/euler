// COPYRIGHT (C) 2022 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use crate::algorithm::long::gcd;
use crate::algorithm::root::square;
use crate::Solver;

/// The points `P (x1, y1)` and `Q (x2, y2)` are plotted at integer co-ordinates and are joined to the origin, `O(0,0)`, to form `ΔOPQ`.
///
/// There are exactly fourteen triangles containing a right angle that can be formed when each co-ordinate lies between `0` and `2` inclusive; that is, `0 ≤ x1, y1, x2, y2 ≤ 2`.
///
/// Given that `0 ≤ x1, y1, x2, y2 ≤ 50`, how many right triangles can be formed?
pub struct Solver091 {
    pub n: i64,
}

impl Default for Solver091 {
    fn default() -> Self {
        Self { n: 50 }
    }
}

impl Solver for Solver091 {
    fn problem_name(&self) -> &str { "Right triangles with integer coordinates" }

    fn solve(&self) -> i64 {
        // there are 3 trivial cases with `n^2` triangles: right angle at the origin and right angle along both axis
        // there are also `n^2/2` triangles with right angle in the main diagonal when `n` is even (and odd, considering integer division)

        // for the general case, consider a point `P` and draw a line perpendicular to `0P`. see how many points `Q` it crosses
        // calculate the slope `x/y` and reduce it to `a/b`. `Q` must be in the form of `(px+k*b, py-k*a)` for some `k!=0`, within the bounds

        7 * square(self.n) / 2 + 2 * (1..=self.n).map(|px| (1..px).map(|py| {
            let factor = gcd(px, py);
            let (a, b) = (px / factor, py / factor);
            (py / a).min((self.n - px) / b) + (px / b).min((self.n - py) / a)
        }).sum::<i64>()).sum::<i64>()
    }
}
