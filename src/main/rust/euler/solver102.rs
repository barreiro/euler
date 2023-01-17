// COPYRIGHT (C) 2022 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::cast::Cast;
use algorithm::io::load_default_data;
use Solver;

/// Three distinct points are plotted at random on a Cartesian plane, for which `-1000 ≤ x, y ≤ 1000`, such that a triangle is formed.
///
/// Consider the following two triangles:
///
/// `A(-340,495)`, `B(-153,-910)`, `C(835,-947)`
/// `X(-175,41)`, `Y(-421,-714)`, `Z(574,-645)`
///
/// It can be verified that triangle `ABC` contains the origin, whereas triangle `XYZ` does not.
///
/// Using triangles.txt (right click and 'Save Link/Target As...'), a 27K text file containing the co-ordinates of one thousand "random" triangles, find the number of triangles for which the interior contains the origin.
///
/// NOTE: The first two examples in the file represent the triangles in the example given above.
pub struct Solver102 {
    pub n: usize,
    pub input: String,
}

impl Default for Solver102 {
    fn default() -> Self {
        Self { n: 1000, input: load_default_data(102) }
    }
}

impl Solver for Solver102 {
    fn solve(&self) -> i64 {
        // checks that both the origin and a point `c` are on the same side of the line that goes through other two points `a` and `b`
        let contains_origin = |t: &Vec<(i64, i64)>| {
            let same_side = |a: (i64, i64), b: (i64, i64), c: (i64, i64)| !(((c.1 - a.1) * (b.0 - a.0) > (b.1 - a.1) * (c.0 - a.0)) ^ (a.1 * (b.0 - a.0) < (b.1 - a.1) * a.0));
            (0..t.len()).all(|i| same_side(t[i], t[(i + 1) % 3], t[(i + 2) % 3]))
        };

        let as_tuples = |line: &str| line.split(',').filter_map(|v| v.parse::<i64>().ok()).collect::<Vec<_>>().chunks(2).map(|x| (x[0], x[1])).collect();
        self.input.lines().take(self.n).map(as_tuples).filter(contains_origin).count().as_i64()
    }
}
