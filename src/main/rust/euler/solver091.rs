// COPYRIGHT (C) 2022 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::long::{gcd, square};
use euler::Solver;

// The points P (x1, y1) and Q (x2, y2) are plotted at integer co-ordinates and are joined to the origin, O(0,0), to form ΔOPQ.
//
// There are exactly fourteen triangles containing a right angle that can be formed when each co-ordinate lies between 0 and 2 inclusive; that is, 0 ≤ x1, y1, x2, y2 ≤ 2.
//
// Given that 0 ≤ x1, y1, x2, y2 ≤ 50, how many right triangles can be formed?

pub struct Solver091 {
    pub n: isize,
}

impl Default for Solver091 {
    fn default() -> Self {
        Solver091 { n: 50 }
    }
}

impl Solver for Solver091 {
    fn solve(&self) -> isize {

        // let is_pythagorean = |(ax, ay), (bx, by)| {
        //     let (sq_a, sq_b, sq_ab) = (square(ax) + square(ay), square(bx) + square(by), square(ax - bx) + square(ay - by));
        //     sq_a + sq_b + sq_ab == sq_a.max(sq_b.max(sq_ab)) << 1
        // };
        //
        // let is_pythagorean = |(ax, ay), (bx, by)| { // dot product between two of the vectors 0P 0Q PQ must be 0
        //     let (cx, cy) = (bx - ax, by - ay);
        //     bx * cx + by * cy == 0 || ax * cx + ay * cy == 0 || ax * bx + ay * by == 0
        // };
        //
        // _points_up_to(self.n).map(|a| _points_up_to(self.n).filter(|&b| a < b && is_pythagorean(a, b)).count() as isize).sum::<isize>()

        // there are 3 trivial cases with n^2 triangles: right angle at the origin and right angle along both axis
        // there are also n^2/2 triangles with right angle in the main diagonal when n is even (and also odd, considering integer division)

        // for the general case, consider a point P and draw a line perpendicular to 0P. see how many points Q it crosses
        // calculate the slope x/y and reduce it to a/b. Q must be in the form of (px+k*b, py-k*a) for some k!=0, within the bounds

        // 7 * square(self.n) / 2 + 2 * (1..=self.n).map(|px| (1..px).map(|py| {
        //     let (a, b) = (px / gcd(px,py), py / gcd(px,py) );
        //     ((1..).take_while(|k| px + k * b <= self.n && py - k * a >= 0).count() + (1..).take_while(|k| px - k * b >= 0 && py + k * a <= self.n).count()) as isize
        // }).sum::<isize>()).sum::<isize>()

        7 * square(self.n) / 2 + 2 * (1..=self.n).map(|px| (1..px).map(|py| {
            let (a, b) = (px / gcd(px, py), py / gcd(px, py));
            (py / a).min((self.n - px) / b) + (px / b).min((self.n - py) / a)
        }).sum::<isize>()).sum::<isize>()
    }
}

// --- //

/// creates an iterator for coordinate points up to certain value n, excluding 0,0
fn _points_up_to(value: isize) -> impl Iterator<Item=(isize,isize)> {
    PointsIterator { x: 0, y: 0, limit: value as _ }
}

struct PointsIterator {
    x: isize,
    y: isize,
    limit: isize,
}

impl Iterator for PointsIterator {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        self.x += 1;
        if self.x > self.limit {
            self.y += 1;
            self.x = 0;
        }
        Some((self.x, self.y)).filter(|_| self.y <= self.limit)
    }
}
