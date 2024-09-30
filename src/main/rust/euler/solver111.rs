// COPYRIGHT (C) 2023 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::iter::from_fn;

use crate::algorithm::cast::to_i64;
use crate::algorithm::combinatorics::{combinations, permutations_with_repetition_of_set};
use crate::algorithm::digits::{from_raw_digits, Digit, DEFAULT_RADIX};
use crate::algorithm::prime::PrimeTestWithCache;
use crate::algorithm::root::pow_10_usize;
use crate::Solver;

/// Considering `4-digit` primes containing repeated digits it is clear that they cannot all be the same: `1111` is divisible by `11`, `2222` is divisible by `22`, and so on.
/// But there are nine `4-digit` primes containing three ones: `1117, 1151, 1171, 1181, 1511, 1811, 2111, 4111, 8111`
///
/// We shall say that `M(n,d)` represents the maximum number of repeated digits for an n-digit prime where `d` is the repeated digit, `N(n,d)` represents the number of such primes, and `S(n,d)` represents the sum of these primes.
///
/// So `M(4,1) == 3` is the maximum number of repeated digits for a `4-digit` prime where `1` is the repeated digit, there are `N(4,1) == 9` such primes, and the sum of these primes is `S(4,1) == 22275`.
/// It turns out that for `d == 0`, it is only possible to have `M(4,0) == 2` repeated digits, but there are `N(4,0) == 13` such cases.
///
/// In the same way we obtain the following results for `4-digit` primes.
/// ```
///  Digit, d  M(4, d)  N(4, d)  S(4, d)
///         0        2       13    67061
///         1        3        9    22275
///         2        3        1     2221
///         3        3       12    46214
///         4        3        2     8888
///         5        3        1     5557
///         6        3        1     6661
///         7        3        9    57863
///         8        3        1     8887
///         9        3        7    48073
/// ```
/// For `d == 0` to `d ==9`, the sum of all `S(4,d) == 273700`.
///
/// Find the sum of all `S(10,d)`.
pub struct Solver111 {
    pub n: usize,
}

impl Default for Solver111 { 
    fn default() -> Self {
        Self { n: 10 }
    }
}

impl Solver for Solver111 {
    fn problem_name(&self) -> &str { "Primes with runs" }

    fn solve(&self) -> i64 {
        let tester = PrimeTestWithCache::new(pow_10_usize(self.n / 2)); // cache can be smaller than default
        let sum_or_primes = |m, d| candidates(d, self.n, m).filter(|&candidate| tester.is_prime(candidate)).map(to_i64).reduce(|sum, value| sum + value);

        // attempt to find primes at decrementing values of `M(n,d)` (starting at `n - 1` or `n - 2` for `d == 0`)
        (0..DEFAULT_RADIX).filter_map(|d| (1..=self.n - if d != 0 { 1 } else { 2 }).rev().find_map(|m| sum_or_primes(m, d))).sum()
    }
}

// --- //

// attempt to find prime candidates by placing a permutation of digits over some combination of positions
fn candidates(base: Digit, dimension: usize, repeated: usize) -> impl Iterator<Item=u64> {
    let mut permutations = permutations_with_repetition_of_set((0..DEFAULT_RADIX).collect(), dimension - repeated).filter(move |p| p.iter().all(|&digit| digit != base));
    let (mut permutation, mut positions) = (permutations.next().expect("There should be an initial permutation"), None);
    from_fn(move || loop {
        if let Some(position) = positions.get_or_insert(combinations((0..dimension).collect(), dimension - repeated)).next() {
            let mut array = vec![base; dimension];
            (0..position.len()).for_each(|i| array[position[i]] = permutation[i]);
            if array[dimension - 1] != 0 && array[0] & 1 != 0 { // skip values starting with zero and even values
                return Some(from_raw_digits(&array));
            }
        } else if let Some(next) = permutations.next() { // reset positions for a new permutation, otherwise end iteration
            (permutation, positions) = (next, None);
        } else {
            return None;
        }
    })
}
