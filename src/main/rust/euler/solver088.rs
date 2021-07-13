// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::bit::BitSet;
use euler::Solver;
use euler::algorithm::long::{floor_sqrt, div_ceil};

// A natural number, N, that can be written as the sum and product of a given set of at least two natural numbers, {a1, a2, ..., ak} is called a product-sum number: N = a1 + a2 + ... + ak = a1 × a2 × ... × ak.
//
// For example, 6 = 1 + 2 + 3 = 1 × 2 × 3.
//
// For a given set of size, k, we shall call the smallest N with this property a minimal product-sum number. The minimal product-sum numbers for sets of size, k = 2, 3, 4, 5, and 6 are as follows.
//
// k=2: 4 = 2 × 2 = 2 + 2
// k=3: 6 = 1 × 2 × 3 = 1 + 2 + 3
// k=4: 8 = 1 × 1 × 2 × 4 = 1 + 1 + 2 + 4
// k=5: 8 = 1 × 1 × 2 × 2 × 2 = 1 + 1 + 2 + 2 + 2
// k=6: 12 = 1 × 1 × 1 × 1 × 2 × 6 = 1 + 1 + 1 + 1 + 2 + 6
//
// Hence for 2 ≤ k ≤ 6, the sum of all the minimal product-sum numbers is 4 + 6 + 8 + 12 = 30; note that 8 is only counted once in the sum.
// In fact, as the complete set of minimal product-sum numbers for 2 ≤ k ≤ 12 is {4, 6, 8, 12, 15, 16}, the sum is 61.
//
// What is the sum of all the minimal product-sum numbers for 2 ≤ k ≤ 12000?

pub struct Solver088 {
    pub n: isize
}

impl Default for Solver088 {
    fn default() -> Self {
        Solver088 { n: 12_000 }
    }
}

impl Solver for Solver088 {
    fn solve(&self) -> isize {
        // see http://www.marmet.org/louis/sumprod/index.html for a list of the actual solutions

        let (mut min_product_sum, mut dedup) = (vec![isize::MAX; self.n as usize + 1], BitSet::new());
        (2..=floor_sqrt(self.n) + 1).for_each(|k| recursive_product_sum(k, k, 1, k, &mut *min_product_sum));
        min_product_sum.iter().skip(2).filter(|&&n| dedup.insert(n)).sum()

        // initial implementation, based on searching on increasing number of fixed digits
        // not very fast as fixed not always increases and not always yields the min product

        // let (mut dedup, mut fixed) = (BitSet::new(), 0);
        // let min_product_sum = |n| {
        //     loop {
        //         if let Some(mut s) = (n..=2*n).find(|&p| exists_sum(p - fixed, p, n - fixed)) {
        //             // search for sums with more fixed (== 1) digits that have smaller products
        //             (1..5).filter_map(|k| (n..=2*n).find(|&p| exists_sum(p - fixed - k, p, n - fixed - k))).for_each(|o| s = s.min(o));
        //             fixed = fixed - if fixed < 3 { 0 } else { 3 }; // sometimes a scale back on the number of fixed digits is required
        //             return s
        //         }
        //         fixed += 1;
        //     }
        // };
        // (2..=self.n).map(min_product_sum).filter(|&n| dedup.insert(n)).sum()
    }
}

// fn exists_sum(s: isize, p: isize, len: isize) -> bool {
//     if len == 1 { s == p } else if p < s { false } else { proper_factors_of(p).filter(|&a| a != 1).any(|a| exists_sum(s - a, p / a, len - 1)) }
// }

// the idea is to have a product `p` and the corresponding sum `s` of size `len`
// there is a product-sum for a set of size `len + (p - s)` with value `p = s + (p - s)`
fn recursive_product_sum(p: isize, s: isize, len: usize, floor: isize, product_sum: &mut [isize]) {
    let index = len + (p - s) as usize;
    product_sum[index] = product_sum[index].min(p);

    // recursively apply with `i >= floor` while maintaining `(p * i) - (s + i) + (len + 1) < product_sum.len()`
    if (product_sum.len() + len + 1) as isize + s >= p * floor {
        (floor..div_ceil((product_sum.len() - len - 1) as isize + s, p - 1)).for_each(|i| recursive_product_sum(p * i, s + i, len + 1, i, product_sum))
    }
}
