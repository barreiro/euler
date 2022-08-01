// COPYRIGHT (C) 2022 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::long::floor_sqrt;
use euler::Solver;

// The proper divisors of a number are all the divisors excluding the number itself.
// For example, the proper divisors of 28 are 1, 2, 4, 7, and 14.
// As the sum of these divisors is equal to 28, we call it a perfect number.
//
// Interestingly the sum of the proper divisors of 220 is 284 and the sum of the proper divisors of 284 is 220, forming a chain of two numbers.
// For this reason, 220 and 284 are called an amicable pair.
//
// Perhaps less well known are longer chains.
// For example, starting with 12496, we form a chain of five numbers: 12496 → 14288 → 15472 → 14536 → 14264 (→ 12496 → ...)
//
// Since this chain returns to its starting point, it is called an amicable chain.
// Find the smallest member of the longest amicable chain with no element exceeding one million.

pub struct Solver095 {
    pub n: isize,
}

impl Default for Solver095 {
    fn default() -> Self {
        Solver095 { n: 1_000_000 }
    }
}

impl Solver for Solver095 {
    fn solve(&self) -> isize {
        // let cache = (0..self.n).map(sum_of_factors).collect::<Vec<_>>();

        // use a kind of sieve to generate a cache for the sum of proper factors
        // let mut cache = vec![1; self.n as usize];
        // (2..self.n >> 1).for_each(|base| (base << 1..self.n).step_by(base as _).for_each(|target| cache[target as usize] += base));
        // cache[0..2].fill(0);

        // use a kind of sieve to generate a cache for the sum of proper factors
        // it is sufficient to stop at sqrt(n) since to any factor above the root there is one below
        let mut cache = vec![1; 1 + self.n as usize];
        cache[0..2].fill(0);
        (2..=floor_sqrt(self.n)).for_each(|i| {
            cache[(i * i) as usize] += i;
            (i + 1..=self.n / i).for_each(|k| cache[(k * i) as usize] += k + i)
        });

        // a slower way to generate the sum of proper factors
        // let cache = (0..self.n).map(|value| (1..=value).map(|i| value - value % i).sum::<isize>() - arithmetic_sum(value)).scan(0, |state, s| {
        //     let previous = *state;
        //     *state = s;
        //     Some(s - previous)
        // }).collect::<Vec<_>>();

        // calculate the chain length for a given value, but only when the start value is the smallest one on the chain
        // let amicable_chain_len = |value| {
        //     let (mut chain, mut sum_factor) = (vec![], value);
        //     loop {
        //         sum_factor = cache[sum_factor as usize];
        //         if sum_factor < value || sum_factor >= self.n || chain.contains(&sum_factor) {
        //             return usize::MIN;
        //         } else if sum_factor == value {
        //             return chain.len() + 1;
        //         } else {
        //             chain.push(sum_factor);
        //         }
        //     }
        // };

        // calculate the chain length for a given value, but only when the start value is the smallest value in the chain
        // using Brent's algorithm defined in https://en.wikipedia.org/wiki/Cycle_detection#Brent's_algorithm
        let amicable_chain_len = |value| {
            let (mut slow, mut fast, mut pow, mut lambda) = (value, cache[value as usize], 1, 1);
            while slow != fast && slow >= value && slow < self.n && fast >= value && fast < self.n {
                if lambda == pow {
                    slow = fast;
                    pow <<= 1;
                    lambda = 0;
                }
                fast = cache[fast as usize];
                lambda += 1;
            }
            if slow == fast { lambda } else { usize::MIN }
        };

        // even numbers usually provide the longest chains. it's safe do do it in the context of this problem
        (0..self.n).step_by(2).max_by_key(|&n| amicable_chain_len(n)).unwrap()
    }
}
