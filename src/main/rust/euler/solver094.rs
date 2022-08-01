// COPYRIGHT (C) 2022 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::long::{is_even, square};
use euler::Solver;

// It is easily proved that no equilateral triangle exists with integral length sides and integral area.
// However, the almost equilateral triangle 5-5-6 has an area of 12 square units.
//
// We shall define an almost equilateral triangle to be a triangle for which two sides are equal and the third differs by no more than one unit.
// Find the sum of the perimeters of all almost equilateral triangles with integral side lengths and area and whose perimeters do not exceed one billion (1,000,000,000).

pub struct Solver094 {
    pub n: isize,
}

impl Default for Solver094 {
    fn default() -> Self {
        Solver094 { n: 1_000_000_000 }
    }
}

impl Solver for Solver094 {
    fn solve(&self) -> isize {

        // solutions alternate between the third side being smaller and larger. the equal sides have odd length.
        // let (mut is_more, is_triplet) = (false, |c, a| is_square(square(c) - square(a)));
        // (3..self.n / 6).filter_map(|a| {
        //     let c = (a << 1) + if is_more { 1 } else { -1 };
        //     Some((c + a) << 1).filter(|_| is_triplet(c, a)).map(|p| {
        //         is_more = !is_more;
        //         p
        //     })
        // }).sum()

        // solutions alternate between the third side being smaller and larger. the equal sides have odd length.
        // let (mut a, mut sum, mut is_more, is_triplet) = (3, 0, false, |c, a| is_square(square(c) - square(a)));
        // while a * 6 < self.n {
        //     let c = (a << 1) + if is_more { 1 } else { -1 };
        //     if is_triplet(c, a) {
        //         is_more = !is_more;
        //         sum += (c + a) << 1;
        //         a = c;
        //     }
        //     a += 1;
        // }
        // sum

        // find Pythagorean triplets a,b,c where abs(c-a*2)==1 give a triangle with perimeter (c+a)*2 and integer area (a*b)
        // pythagorean_triplets().take_while(|&(a, _, _)| a * 6 < self.n).filter(|&(a, _, c)| (c - (a << 1)).abs() == 1).map(|(a, _, c)| (a + c) << 1).sum()

        // Every perimeter is the square of a number or double that, with every second member being a "double". The numbers which are squared are 4,5,14,19,52,71,...
        // Fibonacci-like: the values at even indices are the sum of the previous two, the ones at odd indices are twice the previous plus the one before that
        fibonacci_like().map(|base| square(base) << if is_even(base) { 0 } else { 1 }).take_while(|&p| p < self.n).sum()
    }
}

// --- //

fn fibonacci_like() -> impl Iterator<Item=isize> {
    FibonacciLike { a: 2, b: 1, even: false }
}

struct FibonacciLike {
    a: isize,
    b: isize,
    even: bool, // alternate between updating a (even) and b (odd)
}

impl Iterator for FibonacciLike {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        self.even = !self.even;
        if self.even {
            self.a += self.b << 1;
            Some(self.a)
        } else {
            self.b += self.a;
            Some(self.b)
        }
    }
}
