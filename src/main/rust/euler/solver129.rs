// COPYRIGHT (C) 2024 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use crate::algorithm::cast::Cast;
use crate::algorithm::long::{are_coprime, Increment};
use crate::Solver;

/// A number consisting entirely of ones is called a repunit. 
///
/// We shall define `R(k)` to be a repunit of length `k`; for example, `R(6) = 111111`.
///
/// Given that `n` is a positive integer and `gcd(n, 10) = 1`, it can be shown that there always exists a value, `k`, for which `R(k)` is divisible by `n`, and let `A(n)` be the least such value of `k`; for example, `A(7) = 6` and `A(41) = 5`.
///
/// The least value of `n` for which `A(n)` first exceeds ten is `17`.
///
/// Find the least value of `n` for which `A(n)` first exceeds one-million.
pub struct Solver129 {
    pub n: u64,
}

impl Default for Solver129 {
    fn default() -> Self {
        Self { n: 1_000_000 }
    }
}

impl Solver for Solver129 {
    fn problem_name(&self) -> &str { "Repunit Divisibility" }
    
    fn solve(&self) -> i64 {
        // from https://en.wikipedia.org/wiki/Repunit: using the pigeon-hole principle it can be shown that for relatively prime natural numbers n and b, there exists a repunit in base-b that is a multiple of n.
        // because of that principle A(n) < n (the remainders go 1, 11, 111, …, 0, 1, 11, 111, …) and the search can start at n (and will quickly find a maximum A(m) = m - 1)

        (self.n..u64::MAX).find(|&n| are_coprime(n, 10) && repunit_divisible(n) > self.n).as_i64()
    }
}

/// least value of `k` for each a repunit `R(k)` is divisible by `n`
#[must_use]
pub fn repunit_divisible(n: u64) -> u64 {
    // instead of calculating R(k) for increasing values of k, resulting in very big numbers, use modular arithmetic to calculate the remainders iteratively
    let (mut count, mut state) = (1, 0);
    while {
        state = (10 * state + 1) % n;
        state != 0
    } { count.increment() }
    count
}