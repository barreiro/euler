// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::collections::HashMap;

use algorithm::cast::Cast;
use algorithm::digits::concatenation;
use algorithm::filter::is_prime;
use algorithm::prime::{prime_sieve, primes_up_to};
use algorithm::root::{pow_10, square_u64};
use algorithm::vec::array_sum_u64;
use Solver;

/// The primes `3, 7, 109, and 673`, are quite remarkable. By taking any two primes and concatenating them in any order the result will always be prime.
/// For example, taking `7` and `109`, both `7109` and `1097` are prime. The sum of these four primes, `792`, represents the lowest sum for a set of four primes with this property.
///
/// Find the lowest sum for a set of five primes for which any two primes concatenate to produce another prime.
pub struct Solver060 {
    pub n: u64,
}

impl Default for Solver060 {
    fn default() -> Self {
        Self { n: 5 }
    }
}

impl Solver for Solver060 {
    fn solve(&self) -> i64 {
        let (mut set, primes) = (vec![], primes_up_to(pow_10(self.n - 1)).collect::<Vec<_>>());
        add_prime_to_set(&mut set, self.n.as_usize(), &primes, &mut HashMap::new());
        array_sum_u64(&set).as_i64()
    }
}

fn add_prime_to_set<'a>(set: &mut Vec<u64>, size: usize, primes: &'a [u64], cache: &mut HashMap<u64, Vec<&'a u64>>) -> bool {
    let last_prime = *primes.last().expect("Primes should not be empty");
    let is_prime = |c| if c < last_prime {
        primes.binary_search(&c).is_ok()
    } else if c < square_u64(last_prime) {
        prime_sieve(c, primes)
    } else {
        is_prime(&c)
    };
    let concatenation_list = |p| primes.iter().filter(|&&prime| prime > p && is_prime(concatenation(p, prime)) && is_prime(concatenation(prime, p))).collect::<Vec<_>>();

    // memoization of the prime concatenations for a 25% speedup, despite increasing code complexity significantly
    set.last().iter().for_each(|&&p| { cache.entry(p).or_insert_with(|| concatenation_list(p)); });

    // closure that takes an element of the set and does the intersection with the concatenations of other elements.
    // the outcome is the primes that form concatenations with all elements of the set. From there, try to increase the size of the set by recursion.
    let candidates = |p| cache.get(p).expect("Cache should be populated").iter().filter(|&c| set.iter().all(|&s| s == *p || cache.get(&s).expect("Cache should be populated").binary_search(c).is_ok())).map(|&&s| s).collect();

    set.last().map_or(primes.to_vec(), candidates).iter().any(|&c| {
        set.push(c);
        if set.len() >= size || add_prime_to_set(set, size, primes, cache) {
            true
        } else {
            set.pop();
            false
        }
    })
}
