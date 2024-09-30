// COPYRIGHT (C) 2024 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use crate::algorithm::cast::Cast;
use crate::algorithm::long::pow_mod;
use crate::algorithm::prime::odd_composites;
use crate::Solver;

/// A number consisting entirely of ones is called a repunit. 
///
/// We shall define `R(k)` to be a repunit of length `k`; for example, `R(6) = 111111`.
///
/// Given that `n` is a positive integer and `gcd(n, 10) = 1`, it can be shown that there always exists a value, `k`, for which `R(k)` is divisible by `n`, and let `A(n)` be the least such value of `k`; for example, `A(7) = 6` and `A(41) = 5`.
///
/// You are given that for all primes, `p > 5`, that `p -1` is divisible by `A(p)`.
///
/// For example, when `p = 41`, `A(41) = 5`, and `40` is divisible by `5`.
///
/// However, there are rare composite values for which this is also true; the first five examples being `91`, `259`, `451`, `481`, and `703`.
///
/// Find the sum of the first twenty-five composite values of `n` for which `gcd(n, 10) = 1` and `n - 1` is divisible by `A(n)`.
pub struct Solver130 {
    pub n: usize,
}

impl Default for Solver130 {
    fn default() -> Self {
        Self { n: 25 }
    }
}

impl Solver for Solver130 {
    fn problem_name(&self) -> &str { "Composites with Prime Repunit Property" }

    fn solve(&self) -> i64 {
        // let tester = PrimeTestWithCache::default();

        // (3..).step_by(2)
        //     .filter(|&n| gcd(n.as_i64(), 10) == 1)
        //     .filter(|&n| !tester.is_prime(n))
        //     .filter(|&n| (n - 1) % repunit_divisible(n) == 0)
        //     .take(self.n)
        //     .sum::<u64>().as_i64()
        
        // according to the paper "The Deceptive Primes to 2 * 10^7" by Richard Francis, Timothy Ray (https://projecteuclid.org/download/pdf_1/euclid.mjms/1570500167)
        // theorem 8: if `n` is a deceptive prime, then `n` is a pseudoprime to base 10 
        // this test, along with the evidence that `n` is not divisible by 3, speed up things considerably because the conditions are enough: no need to compute `A(n)`
        
        odd_composites().filter(|&n| n % 3 != 0 && pow_mod(10, n - 1, n) == 1).take(self.n).sum::<u64>().as_i64()
    }
}
