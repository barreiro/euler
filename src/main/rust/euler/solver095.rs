// COPYRIGHT (C) 2022 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::cast::{Cast, UCast};
use algorithm::root::floor_sqrt_u64;
use Solver;

/// The proper divisors of a number are all the divisors excluding the number itself.
/// For example, the proper divisors of `28` are `1, 2, 4, 7, and 14`.
/// As the sum of these divisors is equal to `28`, we call it a perfect number.
///
/// Interestingly the sum of the proper divisors of `220` is `284` and the sum of the proper divisors of `284` is `220`, forming a chain of two numbers.
/// For this reason, `220` and `284` are called an amicable pair.
///
/// Perhaps less well known are longer chains.
/// For example, starting with `12496`, we form a chain of five numbers: `12496 → 14288 → 15472 → 14536 → 14264 (→ 12496 → ...)`
///
/// Since this chain returns to its starting point, it is called an amicable chain.
/// Find the smallest member of the longest amicable chain with no element exceeding one million.
pub struct Solver095 {
    pub n: u64,
}

impl Default for Solver095 {
    fn default() -> Self {
        Self { n: 1_000_000 }
    }
}

impl Solver for Solver095 {
    fn solve(&self) -> i64 {
        // use a kind of sieve to generate a cache for the sum of proper factors
        // it is sufficient to stop at sqrt(n) since to any factor above the root there is one below
        let mut cache = vec![1; self.n.as_usize() + 1];
        cache[0..2].fill(0);
        (2..=floor_sqrt_u64(self.n)).for_each(|i| {
            cache[(i * i).as_usize()] += i;
            (i + 1..=self.n / i).for_each(|k| cache[(k * i).as_usize()] += k + i);
        });

        // calculate the chain length for a given value, but only when the start value is the smallest value in the chain
        // using Brent's algorithm defined in https://en.wikipedia.org/wiki/Cycle_detection#Brent's_algorithm
        let amicable_chain_len = |&value : &u64| {
            let (mut slow, mut fast, mut pow, mut lambda) = (value, cache[value.as_usize()], 1, 1);
            while slow != fast && slow >= value && slow < self.n && fast >= value && fast < self.n {
                if lambda == pow {
                    slow = fast;
                    pow *= 2;
                    lambda = 0;
                }
                fast = cache[fast.as_usize()];
                lambda += 1;
            }
            if slow == fast { Some(lambda) } else { None }
        };

        // even numbers usually provide the longest chains. it's safe do do it in the context of this problem
        (0..self.n).step_by(2).max_by_key(amicable_chain_len).as_i64()
    }
}
