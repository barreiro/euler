// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use crate::algorithm::bit::BitSet;
use crate::algorithm::cast::Cast;
use crate::algorithm::long::div_ceil;
use crate::algorithm::root::ceil_sqrt;
use crate::Solver;

/// A natural number, `N`, that can be written as the sum and product of a given set of at least two natural numbers, `{a1, a2, ..., ak}` is called a product-sum number: `N = a1 + a2 + ... + ak = a1 × a2 × ... × ak`.
///
/// For example, `6 = 1 + 2 + 3 = 1 × 2 × 3`.
///
/// For a given set of size, `k`, we shall call the smallest `N` with this property a minimal product-sum number. The minimal product-sum numbers for sets of size, `k = 2, 3, 4, 5, and 6` are as follows.
/// ```
/// k=2: 4 = 2 × 2 = 2 + 2
/// k=3: 6 = 1 × 2 × 3 = 1 + 2 + 3
/// k=4: 8 = 1 × 1 × 2 × 4 = 1 + 1 + 2 + 4
/// k=5: 8 = 1 × 1 × 2 × 2 × 2 = 1 + 1 + 2 + 2 + 2
/// k=6: 12 = 1 × 1 × 1 × 1 × 2 × 6 = 1 + 1 + 1 + 1 + 2 + 6
/// ```
/// Hence, for `2 ≤ k ≤ 6`, the sum of all the minimal product-sum numbers is `4 + 6 + 8 + 12 = 30`; note that `8` is only counted once in the sum.
/// In fact, as the complete set of minimal product-sum numbers for `2 ≤ k ≤ 12` is `{4, 6, 8, 12, 15, 16}`, the sum is `61`.
///
/// What is the sum of all the minimal product-sum numbers for `2 ≤ k ≤ 12000`?
pub struct Solver088 {
    pub n: usize,
}

impl Default for Solver088 {
    fn default() -> Self {
        Self { n: 12_000 }
    }
}

impl Solver for Solver088 {
    fn problem_name(&self) -> &str { "Product-sum numbers" }

    // see http://www.marmet.org/louis/sumprod/index.html for a list of the actual solutions
    fn solve(&self) -> i64 {
        let (mut min_product_sum, mut dedup) = (vec![i64::MAX; self.n + 1], BitSet::new());
        (2..=ceil_sqrt(self.n.as_i64())).for_each(|k| recursive_product_sum(k, k, 1, k, &mut min_product_sum));
        min_product_sum.into_iter().skip(2).filter(|&n| dedup.insert(n.as_u64())).sum()
    }
}

// the idea is to have a product `p` and the corresponding sum `s` of size `len`
// there is a product-sum for a set of size `len + (p - s)` with value `p = s + (p - s)`
fn recursive_product_sum(p: i64, s: i64, len: usize, floor: i64, product_sum: &mut [i64]) {
    let index = len + (p - s).as_usize();
    product_sum[index] = product_sum[index].min(p);

    // recursively apply with `i >= floor` while maintaining `(p * i) - (s + i) + (len + 1) < product_sum.len()`
    if (product_sum.len() + len + 1).as_i64() + s >= p * floor {
        (floor..div_ceil((product_sum.len() - len - 1).as_i64() + s, p - 1)).for_each(|i| recursive_product_sum(p * i, s + i, len + 1, i, product_sum));
    }
}
