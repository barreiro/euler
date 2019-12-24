// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::long::{floor_sqrt, is_odd, square};
use euler::Solver;

// All square roots are periodic when written as continued fractions and can be written in the form:
// √N = a0 + ( 1 / ( a1 + 1 / ( a2 + 1 / ( a3 + ...
//
// For example, let us consider √23: √23 = 4 + √23 − 4 = 4 + 1 / 1 / ( √23 − 4 ) = 4 + 1 / ( 1 + ( √23 − 3 ) / 7 )
// If we continue we would get the following expansion:
// √23 = 4 + 1 / ( 1 + 1 / ( 3 + 1 / ( 1 + 1 / ( 8 + ...
//
// The process can be summarised as follows:
// a0 = 4 , 1 / ( √23 − 4 ) = ( √23 + 4 ) / 7 = 1 + ( √23 − 3 ) / 7
// a1 = 1 , 7 / ( √23 − 3 ) = 7 * ( √23 + 3 ) / 14 = 3 + ( √23 − 3 ) / 2
// a2 = 3 , 2 / ( √23 − 3 ) = 2 * ( √23 + 3 ) / 14 = 1 + ( √23 − 4 ) / 7
// a3 = 1 , 7 / ( √23 − 4 ) = 7 * ( √23 + 4 ) / 7 = 8 + √23 - 4
// a4 = 8 , 1 / ( √23 − 4 ) = ( √23 + 4 ) / 7 = 1 + ( √23 − 3 ) / 7
// a5 = 1 , 7 / ( √23 − 3 ) = 7 * ( √23 + 3 ) / 14 = 3 + ( √23 − 3 ) / 2
// a6 = 3 , 2 / ( √23 − 3 ) = 2 * ( √23 + 3 ) / 14 = 1 + ( √23 − 4 ) / 7
// a7 = 1 , 7 / ( √23 − 4 ) = 7 * ( √23 + 4 ) / 7 = 8 + √23 − 4
//
// It can be seen that the sequence is repeating. For conciseness, we use the notation √23 = [ 4; ( 1, 3, 1, 8 ) ], to indicate that the block ( 1, 3, 1, 8 ) repeats indefinitely.
//
// The first ten continued fraction representations of (irrational) square roots are:
// √2 = [ 1; ( 2 ) ], period=1
// √3 = [ 1; ( 1, 2 ) ], period=2
// √5 = [ 2; ( 4 ) ], period=1
// √6 = [ 2; ( 2, 4 ) ], period=2
// √7 = [ 2; ( 1, 1, 1, 4 ) ], period=4
// √8 = [ 2; ( 1, 4 ) ], period=2
// √10 = [ 3; ( 6 ) ], period=1
// √11 = [ 3; ( 3, 6 ) ], period=2
// √12 = [ 3; ( 2, 6 ) ], period=2
// √13 = [ 3; ( 1, 1, 1, 1, 6 ) ], period=5
//
// Exactly four continued fractions, for N ≤ 13, have an odd period.
// How many continued fractions for N ≤ 10000 have an odd period?

pub struct Solver064 {
    pub n: isize
}

impl Default for Solver064 {
    fn default() -> Self {
        Solver064 { n: 10_000 }
    }
}

impl Solver for Solver064 {
    fn solve(&self) -> isize {
        // num / (√n - fractional) => a + (√n - b) / c
        let transform = |n, floor, num, fractional| {
            let c = (n - square(fractional)) / num;
            let a = (fractional + floor) / c;
            let b = -fractional + c * a;
            (a, b, c)
        };

        // length of cycle, by applying transformations until c == 1
        let cycle = |n| {
            let floor = floor_sqrt(n);
            if square(floor) == n {
                return 0;
            }
            let (mut num, mut fractional) = (1, floor);
            (0..n).take_while(|_| {
                let (_, b, c) = transform(n, floor, num, fractional);
                num = c;
                fractional = b;
                num != 1
            }).count() as isize + 1
        };

        (1..=self.n).filter(|&n| is_odd(cycle(n))).count() as _
    }
}
