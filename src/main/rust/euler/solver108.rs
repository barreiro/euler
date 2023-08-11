// COPYRIGHT (C) 2022 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::collections::HashMap;
use std::iter::from_fn;

use algorithm::cast::Cast;
use algorithm::filter::less_than_u64;
use algorithm::long::IncrementAndGet;
use algorithm::prime::primorals;
use algorithm::root::square_u64;
use Solver;

/// In the following equation `x`, `y`, and `n` are positive integers.
/// ```
/// 1 / x + 1 / y = 1 / n
/// ```
/// For `n = 4` there are exactly three distinct solutions:
/// ```
/// 1 / 5 + 1 / 20 = 1 / 4
/// 1 / 6 + 1 / 12 = 1 / 4
/// 1 / 8 + 1 /  8 = 1 / 4
/// ```
/// What is the least value of `n` for which the number of distinct solutions exceeds one-thousand?
///
/// NOTE: This problem is an easier version of Problem 110; it is strongly advised that you solve this one first.
pub struct Solver108 {
    pub n: u64,
}

impl Default for Solver108 {
    fn default() -> Self {
        Self { n: 1000 }
    }
}

impl Solver for Solver108 {
    fn solve(&self) -> i64 {
        // let _solutions_a = |n: i64| (0..n).filter(|&y| (n * y) % (n - y) == 0).count();
        // let _solutions_b = |n: i64| proper_factors_of(square(n)).filter(|&f| f < n).count();
        // let _solutions_c = |n: i64| (1 + number_of_factors(square(n))) >> 1;
        // (2..).skip(2).find(|&n| _solutions_c(n) > self.n).as_i64()

        // the number of unit fraction sums that equal `1/n` is the number of divisors of `n^2` (divided by two because we want pairs, not divisors)
        // the number of divisors is maximized by the so called 'Highly Composite Numbers' HCN --- we approximate these using products of primorals
        // assuming `n` has prime factorization `p1^a1, p2^a2, ... pn^an` the number of factors of `n^2` is `(2*a1 + 1)(2*a2 + 1) ... (2*an + 1)`
        highly_composite(square_u64(self.n)).find_map(|(n, factors)| (factors.values().map(|exp| exp * 2 + 1).product::<u64>() > self.n * 2).then_some(n)).as_i64()
    }
}

// creates an iterator of increasing numbers that are multiples of primorals and therefore have 'highly composition'
// the multiples are themselves highly composite numbers previously found, which increase the 'composability' of the results
// these do not match the definition of 'highly composite numbers' but are an approximation good enough for this problem
// together with the numbers there is an efficient generation of the prime factorization
fn highly_composite(upper_bound : u64) -> impl Iterator<Item=(u64, HashMap<u64, u64>)> {
    // the factorization is not on te prime, but on the prime index
    let mut composite_cache = vec![(2u64, HashMap::from([(0, 1)])), (6, HashMap::from([(0, 1),(1, 1)])), (8, HashMap::from([(0, 3)]))];

    primorals().take_while(less_than_u64(upper_bound)).enumerate().for_each(|(n, primoral)| {
        (0..composite_cache.len()).for_each(|i| {
            let (composite, factorization) = &composite_cache[i];
            if let Some(product) = composite.checked_mul(primoral) { // skip products that overflow
                let mut product_factorization = factorization.clone();
                (0..=n).for_each(|i| { product_factorization.entry(i.as_u64()).and_modify(|v| { v.increment_and_get(); }).or_insert(1); });
                composite_cache.push((product, product_factorization));
            }
        });
    });
    composite_cache.sort_unstable_by_key(|&(n, _)| n);
    let mut composite_iter = composite_cache.into_iter();
    from_fn(move || composite_iter.next())
}
