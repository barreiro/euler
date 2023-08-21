// COPYRIGHT (C) 2023 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::collections::HashMap;

use algorithm::cast::Cast;
use algorithm::combinatorics::{partitions_of_set, permutations_of_set_with};
use algorithm::digits::{DEFAULT_RADIX, Digit, from_raw_digits};
use algorithm::prime::{miller_rabin, prime_sieve, primes_up_to};
use algorithm::root::{pow_10, square_u64};
use Solver;

const SEARCH_THRESHOLD: u64 = pow_10(5);
const SIEVE_THRESHOLD: u64 = square_u64(SEARCH_THRESHOLD);

/// Using all of the digits `1` through `9` and concatenating them freely to form decimal integers, different sets can be formed.
///
/// Interestingly with the set `{ 2, 5, 47, 89, 631 }`, all of the elements belonging to it are prime.
///
/// How many distinct sets containing each of the digits one through nine exactly once contain only prime elements?
pub struct Solver118 {
    pub n: Digit,
}

impl Default for Solver118 {
    fn default() -> Self {
        Self { n: DEFAULT_RADIX - 1 }
    }
}

impl Solver for Solver118 {
    fn solve(&self) -> i64 {
        // cache some primes to speed up primality test
        let primes = primes_up_to(SEARCH_THRESHOLD).collect::<Vec<_>>();
        let prime_test = |value| if value < SEARCH_THRESHOLD { primes.binary_search(&value).is_ok() } else if value < SIEVE_THRESHOLD { prime_sieve(value, &primes) } else { miller_rabin(value) };

        // verify if the sets are increasing to remove duplicates.
        // let is_increasing = |elements: &Vec<_>| (1..elements.len()).all(|i| elements[i] < elements[i - 1]);

        // permutations_of_digits_with(1, self.n, |p| p_prime_partitions(p, &prime_test)).flatten().filter(is_increasing).count().as_i64()
        // permutations_of_digits_with(1, self.n, |p| _prime_partitions(p, 1, &prime_test)).flatten().count().as_i64()

        // calculate the number of primes that a partition generates by multiplying the number of primes in permutations of each partition element
        let (mut cache, prime_permutations) = (HashMap::new(), |p: &Vec<Digit>| permutations_of_set_with(p.to_vec(), |permutation| prime_test(from_raw_digits(permutation)).then_some(())).count());
        partitions_of_set(&(1..=self.n).collect::<Vec<_>>()).map(|partition| partition.iter().map(|p| cache.entry(from_raw_digits(p)).or_insert_with(|| prime_permutations(p)).as_i64()).product::<i64>()).sum()
    }
}

// --- //

// find prime partitions (if they exist, and there can be multiple) of a given permutation of digits
// verifies some 'head' of the permutation is prime, and then append to the result of the same function applied to the 'tail'
// fn p_prime_partitions(permutation: &[Digit], prime_test: &dyn Fn(u64) -> bool) -> Option<Vec<Vec<u64>>> {
//     if permutation.is_empty() {
//         Some(vec![])
//     } else {
//         let mut result = vec![];
//         (1..=permutation.len()).fold(0, |mut candidate, i| {
//             // speedup: check for multiples of 2, 3 and 5
//             candidate = candidate * DEFAULT_RADIX.as_u64() + permutation[i - 1].as_u64();
//             if candidate == 2 || candidate == 3 || candidate == 5 || candidate & 1 != 0 && candidate % 3 != 0 && candidate % 5 != 0 && prime_test(candidate) {
//                 if i == permutation.len() {
//                     result.push(vec![candidate]);
//                 } else if let Some(mut next) = p_prime_partitions(&permutation[i..], prime_test) {
//                     next.iter_mut().for_each(|n| n.push(candidate));
//                     result.append(&mut next);
//                 }
//             }
//             candidate
//         });
//         (!result.is_empty()).then_some(result)
//     }
// }

// find prime partitions (if they exist, and there can be multiple) of a given permutation of digits
// verifies some 'head' of the permutation is prime, and then append to the result of the same function applied to the 'tail'
// do not report duplicate partitions by making sure that the list of the primes is always decreasing
fn _prime_partitions(permutation: &[Digit], min_size: usize, prime_test: &dyn Fn(u64) -> bool) -> Option<Vec<Vec<u64>>> {
    if permutation.is_empty() {
        Some(vec![])
    } else {
        let mut result = vec![];
        (min_size..=permutation.len()).for_each(|i| {
            let candidate = from_raw_digits(&permutation[0..i]);
            // speedup by not performing primality test for multiples of 2, 3 and 5
            if candidate == 2 || candidate == 3 || candidate == 5 || candidate & 1 != 0 && candidate % 3 != 0 && candidate % 5 != 0 && prime_test(candidate) {
                if i == permutation.len() {
                    result.push(vec![candidate]);
                } else if let Some(mut next) = _prime_partitions(&permutation[i..], i, prime_test) {
                    next.retain(|n| n.last().iter().any(|&&last| last > candidate));
                    next.iter_mut().for_each(|n| n.push(candidate));
                    result.append(&mut next);
                }
            }
        });
        (!result.is_empty()).then_some(result)
    }
}
