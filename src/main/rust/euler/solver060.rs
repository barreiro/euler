// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::collections::HashMap;

use euler::algorithm::long::{concatenation, pow_10, square};
use euler::algorithm::prime::{generator_wheel, miller_rabin, prime_sieve};
use euler::Solver;

// The primes 3, 7, 109, and 673, are quite remarkable. By taking any two primes and concatenating them in any order the result will always be prime.
// For example, taking 7 and 109, both 7109 and 1097 are prime. The sum of these four primes, 792, represents the lowest sum for a set of four primes with this property.
//
// Find the lowest sum for a set of five primes for which any two primes concatenate to produce another prime.

pub struct Solver060 {
    pub n: isize
}

impl Default for Solver060 {
    fn default() -> Self {
        Solver060 { n: 5 }
    }
}

impl Solver for Solver060 {
    fn solve(&self) -> isize {
        let (mut set, primes) = (vec![], generator_wheel().take_while(|&p| p < pow_10(self.n - 1)).collect::<Vec<_>>());
        add_prime_to_set(&mut set, self.n as _, &primes, &mut HashMap::new());
        set.iter().sum()
    }
}

fn add_prime_to_set<'a>(set: &mut Vec<isize>, size: usize, primes: &'a [isize], cache: &mut HashMap<isize, Vec<&'a isize>>) -> bool {
    let last_prime = *primes.last().unwrap();
    let is_prime = |c| if c < last_prime {
        primes.binary_search(&c).is_ok()
    } else if c < square(last_prime) {
        prime_sieve(c, primes)
    } else {
        miller_rabin(c)
    };
    let concatenation_list = |p| primes.iter().filter(|&&prime| prime > p && is_prime(concatenation(p, prime)) && is_prime(concatenation(prime, p))).collect::<Vec<_>>();

    // Memoization of the prime concatenations for a 25% speedup, despite increasing code complexity significantly
    set.last().iter().for_each(|&&p| { cache.entry(p).or_insert_with(|| concatenation_list(p)); });

    // Closure that takes an element of the set and does the intersection with the concatenations of other elements.
    // The outcome is the primes that form concatenations with all elements of the set. From there, try to increase the size of the set by recursion.
    let candidates = |p| cache.get(p).unwrap().iter().filter(|&c| set.iter().all(|&s| s == *p || cache.get(&s).unwrap().binary_search(c).is_ok())).map(|&&s| s).collect();

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
