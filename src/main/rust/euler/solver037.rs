// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::bit::BitSet;
use algorithm::cast::to_i64;
use algorithm::digits::DEFAULT_RADIX;
use algorithm::prime::generator_wheel;
use Solver;

const BASE: u64 = DEFAULT_RADIX as u64;

/// The number `3797` has an interesting property. Being prime itself, it is possible to continuously remove digits from left to right, and remain prime at each stage: `3797, 797, 97, and 7`.
/// Similarly we can work from right to left: `3797, 379, 37, and 3`.
/// Find the sum of the only eleven primes that are both truncatable from left to right and right to left.
/// NOTE: `2, 3, 5, and 7` are not considered to be truncatable primes.
pub struct Solver037 {
    pub n: usize,
}

impl Default for Solver037 {
    fn default() -> Self {
        Self { n: 11 }
    }
}

impl Solver for Solver037 {
    fn solve(&self) -> i64 {
        let mut set = BitSet::new();

        let is_truncatable_right = |p, set: &BitSet| {
            let mut l = p / BASE;
            while l > 0 {
                if !set.contains(l) {
                    return false;
                }
                l /= BASE;
            }
            true
        };

        let is_truncatable_left = |p, set: &BitSet| {
            let (mut l, mut n) = (p % BASE, BASE);
            while l < p {
                if !set.contains(l) {
                    return false;
                }
                n *= BASE;
                l = p % n;
            }
            true
        };

        generator_wheel().filter(|&p| set.insert(p) && is_truncatable_right(p, &set) && is_truncatable_left(p, &set) && p > BASE).take(self.n).map(to_i64).sum()
    }
}
