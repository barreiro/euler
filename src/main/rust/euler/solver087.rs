// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::bit::BitSet;
use algorithm::cast::Cast;
use algorithm::filter::less_than_u64;
use algorithm::prime::primes_up_to;
use algorithm::root::{cube_u64, floor_sqrt_u64, fourth_u64, square_u64};
use Solver;

/// The smallest number expressible as the sum of a prime square, prime cube, and prime fourth power is `28`.
/// In fact, there are exactly four numbers below fifty that can be expressed in such a way:
/// ```
/// 28 = 2^2 + 2^3 + 2^4
/// 33 = 3^2 + 2^3 + 2^4
/// 49 = 5^2 + 2^3 + 2^4
/// 47 = 2^2 + 3^3 + 2^4
/// ```
/// How many numbers below fifty million can be expressed as the sum of a prime square, prime cube, and prime fourth power?
pub struct Solver087 {
    pub n: u64,
}

impl Default for Solver087 {
    fn default() -> Self {
        Self { n: 50_000_000 }
    }
}

impl Solver for Solver087 {
    fn problem_name(&self) -> &str { "Prime power triples" }

    fn solve(&self) -> i64 {
        // the exact bound is p <= floor_sqrt(n - 24), but as n >>> 24 can be approached by p < floor_sqrt(n)
        let (primes, mut solutions) = (primes_up_to(floor_sqrt_u64(self.n)).collect::<Vec<_>>(), BitSet::new());
        let primes_apply = |action: fn(u64) -> u64| primes.iter().map(|&p| action(p)).take_while(less_than_u64(self.n)).collect::<Vec<_>>();
        let (squares, cubes, fourths) = (primes_apply(square_u64), primes_apply(cube_u64), primes_apply(fourth_u64));

        for f in fourths {
            cubes.iter().map(|c| c + f).take_while(|&cf| cf < self.n).for_each(|cf| {
                squares.iter().map(|&s| s + cf).take_while(|&scf| scf < self.n).for_each(|scf| { solutions.insert(scf); });
            });
        }

        solutions.len().as_i64()
    }
}
