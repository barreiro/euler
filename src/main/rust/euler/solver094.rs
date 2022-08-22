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
