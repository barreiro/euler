// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::cast::Cast;
use algorithm::prime::prime_sieve;
use algorithm::root::is_square;
use Solver;

/// It was proposed by Christian Goldbach that every odd composite number can be written as the sum of a prime and twice a square.
///
/// ` 9 =  7 + 2*1^2`
/// `15 =  7 + 2*2^2`
/// `21 =  3 + 2*3^2`
/// `25 =  7 + 2*3^2`
/// `27 = 19 + 2*2^2`
/// `33 = 31 + 2*1^2`
///
/// It turns out that the conjecture was false.
/// What is the smallest odd composite that cannot be written as the sum of a prime and twice a square?
pub struct Solver046 {
    pub n: usize
}

impl Default for Solver046 {
    fn default() -> Self {
        Self { n: 1 }
    }
}

impl Solver for Solver046 {
    fn solve(&self) -> i64 {
        let mut primes = vec![2];
        (3..).step_by(2).filter(|&i| {
            if prime_sieve(i, &primes) {
                primes.push(i);
                return false;
            }
            !primes.iter().rev().any(|&p| is_square(((i - p) / 2).as_i64()))
        }).nth(self.n - 1).as_i64()
    }
}
