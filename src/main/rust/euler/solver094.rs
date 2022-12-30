// COPYRIGHT (C) 2022 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::iter::from_fn;

use algorithm::cast::Cast;
use algorithm::filter::{is_even_u64, less_than_u64};
use algorithm::root::square_u64;
use Solver;

/// It is easily proved that no equilateral triangle exists with integral length sides and integral area.
/// However, the almost equilateral triangle `5-5-6` has an area of `12` square units.
///
/// We shall define an almost equilateral triangle to be a triangle for which two sides are equal and the third differs by no more than one unit.
/// Find the sum of the perimeters of all almost equilateral triangles with integral side lengths and area and whose perimeters do not exceed one billion (`1,000,000,000`).
pub struct Solver094 {
    pub n: u64,
}

impl Default for Solver094 {
    fn default() -> Self {
        Self { n: 1_000_000_000 }
    }
}

impl Solver for Solver094 {
    fn solve(&self) -> i64 {
        // find Pythagorean triplets a,b,c where abs(c-a*2)==1 give a triangle with perimeter (c+a)*2 and integer area (a*b)
        // pythagorean_triplets().take_while(|&(a, _, _)| a * 6 < self.n).filter(|&(a, _, c)| (c - (a << 1)).abs() == 1).map(|(a, _, c)| (a + c) << 1).sum()

        // every perimeter is the square of a number or double that, with every second member being a "double". The numbers which are squared are 4,5,14,19,52,71,...
        // Fibonacci-like: the values at even indices are the sum of the previous two, the ones at odd indices are twice the previous plus the one before that
        fibonacci_like().map(|base| square_u64(base) << if is_even_u64(&base) { 0 } else { 1 }).take_while(less_than_u64(self.n)).sum::<u64>().as_i64()
    }
}

// --- //

fn fibonacci_like() -> impl Iterator<Item=u64> {
    // even alternate between updating a (even) and b (odd)
    let (mut a, mut b, mut even) = (2, 1, false);
    from_fn(move || {
        even = !even;
        if even {
            a += b * 2;
            Some(a)
        } else {
            b += a;
            Some(b)
        }
    })
}
