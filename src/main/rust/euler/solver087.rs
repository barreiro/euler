// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::bit::BitSet;
use euler::algorithm::long::{cube, floor_sqrt, fourth, square};
use euler::algorithm::prime::primes_up_to;
use euler::Solver;

// The smallest number expressible as the sum of a prime square, prime cube, and prime fourth power is 28.
// In fact, there are exactly four numbers below fifty that can be expressed in such a way:
//
// 28 = 2^2 + 2^3 + 2^4
// 33 = 3^2 + 2^3 + 2^4
// 49 = 5^2 + 2^3 + 2^4
// 47 = 2^2 + 3^3 + 2^4
//
// How many numbers below fifty million can be expressed as the sum of a prime square, prime cube, and prime fourth power?

pub struct Solver087 {
    pub n: isize
}

impl Default for Solver087 {
    fn default() -> Self {
        Solver087 { n: 50_000_000 }
    }
}

impl Solver for Solver087 {
    fn solve(&self) -> isize {
        // the exact bound is p <= floor_sqrt(n - 24), but as n >>> 24 can be approached by p < floor_sqrt(n)
        let (primes, mut solutions) = (primes_up_to(floor_sqrt(self.n)).collect::<Vec<_>>(), BitSet::new());
        let primes_apply = |action: fn(isize) -> isize| primes.iter().map(|&p| action(p)).take_while(|&p| p < self.n).collect::<Vec<_>>();
        let (squares, cubes, fourths) = (primes_apply(square), primes_apply(cube), primes_apply(fourth));

        fourths.iter().for_each(|&f| {
            cubes.iter().map(|c| c + f).take_while(|&cf| cf < self.n).for_each(|cf| {
                squares.iter().map(|&s| s + cf).take_while(|&scf| scf < self.n).for_each(|scf| { solutions.insert(scf); })
            })
        });

        solutions.len() as _
    }
}
