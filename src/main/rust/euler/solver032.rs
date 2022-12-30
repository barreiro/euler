// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::bit::BitSet;
use algorithm::cast::to_i64;
use algorithm::combinatorics::permutations_of_digits_with;
use algorithm::digits::from_raw_digits;
use Solver;

/// We shall say that an n-digit number is pandigital if it makes use of all the digits `1` to `n` exactly once; for example, the 5-digit number, `15234`, is `1` through `5` pandigital.
/// The product `7254` is unusual, as the identity, `39 × 186 = 7254`, containing multiplicand, multiplier, and product is `1` through `9` pandigital.
/// Find the sum of all products whose multiplicand/multiplier/product identity can be written as a `1` through `9` pandigital.
/// HINT: Some products can be obtained in more than one way so be sure to only include it once in your sum.
pub struct Solver032 {
    pub n: u8,
}

impl Default for Solver032 {
    fn default() -> Self {
        Self { n: 9 }
    }
}

impl Solver for Solver032 {
    fn solve(&self) -> i64 {
        let mut set = BitSet::new();
        permutations_of_digits_with(1, self.n, |p| {
            // assume that the product is the first half of the digits and the factors the other half
            let (half, quarter) = (p.len() / 2, p.len() / 4);
            (half + 1..=half + quarter).find_map(|j| {
                let (a, b, c) = (from_raw_digits(&p[0..half]), from_raw_digits(&p[half..j]), from_raw_digits(&p[j..]));
                Some(a).filter(|_| a == b * c)
            })
        }).filter(|&a| set.insert(a)).map(to_i64).sum()
    }
}
