// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::convert::TryFrom;

use crate::algorithm::cast::Cast;
use crate::algorithm::root::floor_sqrt_u64;
use crate::Solver;

/// The first two consecutive numbers to have two distinct prime factors are:
/// ```
/// 14 = 2 × 7
/// 15 = 3 × 5
/// ```
/// The first three consecutive numbers to have three distinct prime factors are:
/// ```
/// 644 = 22 × 7 × 23
/// 645 = 3 × 5 × 43
/// 646 = 2 × 17 × 19
/// ```
/// Find the first four consecutive integers to have four distinct prime factors each. What is the first of these numbers?
pub struct Solver047 {
    pub n: u64,
}

impl Default for Solver047 {
    fn default() -> Self {
        Self { n: 4 }
    }
}

impl Solver for Solver047 {
    fn problem_name(&self) -> &str { "Distinct primes factors" }

    #[allow(clippy::maybe_infinite_iter)]
    fn solve(&self) -> i64 {
        let mut primes = vec![2];
        (3..).scan(0, |count, l| {
            *count = if is_num_prime_factors(l, &mut primes, self.n) { *count + 1 } else { 0 };
            Some( if *count == self.n { l - self.n + 1 } else { 0 } )
        }).find(|&a| a != 0).as_i64()
    }
}

// Similar to primes.prime_factors() but optimized for this problem
fn is_num_prime_factors(n: u64, primes: &mut Vec<u64>, expected: u64) -> bool {
    let (mut count, mut value, small, stop) = (0, n, i32::try_from(n).is_ok(), floor_sqrt_u64(n));
    for &factor in primes.iter() {
        let mut divides = false;
        while if small { i32::try_from(value).expect("Value should be small") % i32::try_from(factor).expect("Value should be small") == 0 } else { value % factor == 0 } {
            value /= factor;
            divides = true;
        }
        if divides {
            count += 1;
            // if count > expected { return false; } is slightly slower
        }
        if factor >= stop {
            // if the number is prime, or if there is still a remainder, add itself as a factor
            if count == 0 {
                primes.push(n);
            }
            if value >= factor || count == 0 {
                count += 1;
            }
            return count == expected;
        }
    }
    false
}
