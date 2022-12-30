// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::cast::Cast;
use algorithm::factor::proper_factors_of;
use algorithm::root::square;
use Solver;

/// A spider, `S`, sits in one corner of a cuboid room, measuring `6` by `5` by `3`, and a fly, `F`, sits in the opposite corner.
/// By travelling on the surfaces of the room the shortest "straight line" distance from `S` to `F` is `10` and the path is shown on the diagram.
///
/// However, there are up to three "shortest" path candidates for any given cuboid and the shortest route doesn't always have integer length.
///
/// It can be shown that there are exactly `2060` distinct cuboids, ignoring rotations, with integer dimensions, up to a maximum size of `M` by `M` by `M`, for which the shortest route has integer length when `M = 100`.
/// This is the least value of `M` for which the number of solutions first exceeds two thousand; the number of solutions when `M = 99` is `1975`.
///
/// Find the least value of `M` such that the number of solutions first exceeds one million.
pub struct Solver086 {
    pub n: i64,
}

impl Default for Solver086 {
    fn default() -> Self {
        Self { n: 1_000_000 }
    }
}

#[allow(clippy::maybe_infinite_iter)]
impl Solver for Solver086 {
    fn solve(&self) -> i64 {
        // for a cuboid a, b, c with a >= b >= c >= 1, the point of shortest path a_min (along the a side) is given by (b * a) / (c + b)
        // the length of the path d is sqrt(a_min^2 + b^2) + sqrt((a - a_min)^2 + c^2), substituting it simplifies to sqrt(a^2 + (b+c)^2)
        // we want to find the number of values s = (b + c) such that a^2 + s^2 is a square (with the constrain a >= b >= c >= 1)

        // we are after factor pairs of a^2 (because a^2 = d^2 - s^2 = (d + s)(d - s)) with parity (to generate integer solutions)
        // given one of these pairs x and y, there are two solutions s = (y + x) / 2 and s = (y - x) / 2
        // since d > s, the only that conform to the bounds is s = (y - x) / 2

        // there are s / 2 different ways for 2 numbers to add up to s and since a >= b >= c, s <= 2 * a
        // we must subtract to that the ways where one of the terms is bigger than a

        let (parity, average) = (|&(x, y): &(_, _)| x & 1 == y & 1, |(x, y)| (y - x) / 2);

        // return the number of cuboids with integer minimum distance == d and longest side == a
        let cuboids = |a| -> i64 {
            let (a_square, bound) = (square(a), |&s: &_| s < a * 2);
            let factor_pair = |x| (x, a_square / x);

            // skip the trivial sqrt solution where both factors are equal
            proper_factors_of(a_square).skip(1).map(factor_pair).filter(parity).map(average).take_while(bound).map(|s| (s >> 1) - 0.max(s - a - 1)).sum()
        };

        // sum the contributions of the cuboids until it goes over the number of solutions required
        (1..).scan(0, |count, m| {
            *count += cuboids(m);
            Some(m + 1).filter(|_| *count <= self.n)
        }).last().as_i64()
    }
}
