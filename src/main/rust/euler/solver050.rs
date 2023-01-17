// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::cast::Cast;
use algorithm::filter::is_prime;
use algorithm::prime::generator_trial_division;
use algorithm::root::pow_10;
use algorithm::vec::array_sum_u64;
use Solver;

/// The prime `41`, can be written as the sum of six consecutive primes: `41 = 2 + 3 + 5 + 7 + 11 + 13`
/// This is the longest sum of consecutive primes that adds to a prime below one-hundred.
/// The longest sum of consecutive primes below one-thousand that adds to a prime, contains `21` terms, and is equal to `953`.
///
/// Which prime, below one-million, can be written as the sum of the most consecutive primes?
pub struct Solver050 {
    pub n: u64,
}

impl Default for Solver050 {
    fn default() -> Self {
        Self { n: 6 }
    }
}

impl Solver for Solver050 {
    fn solve(&self) -> i64 {
        // the list of primes which sum is below the limit, then starting on the greater window sizes try to find a sum that is prime
        let primes = generator_trial_division().scan(0, |acc, p| {
            *acc += p;
            Some(p).filter(|_| *acc < pow_10(self.n))
        }).collect::<Vec<_>>();

        (1..=primes.len()).rev().find_map(|len| primes.windows(len).rev().map(array_sum_u64).find(is_prime)).as_i64()
    }
}
