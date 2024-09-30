// COPYRIGHT (C) 2024 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::collections::BTreeMap;
use std::iter::from_fn;

use crate::algorithm::cast::Cast;
use crate::algorithm::long::are_coprime;
use crate::algorithm::prime::radicals_up_to;
use crate::Solver;

/// The radical of `n`, `rad(n)`, is the product of distinct prime factors of `n`.
///
/// For example, `504 = 2^3 * 3^2 * 7`, so `rad(504) = 2 * 3 * 7 = 42`.
///
/// We shall define the triplet of positive integers `(a, b, c)` to be an abc-hit if:
///```
/// 1 - gdc(a,b) = gdc(a,c) = gdc(b,c) = 1
/// 2 - a < b
/// 3 - a + b = c
/// 4 - rad(abc) < c
///```
/// For example, `(5, 27, 32)` is an abc-hit, because:
///```
/// 1 - gdc(5, 27) = gdc(5, 32) = gdc(27,32) = 1
/// 2 - 5 < 27
/// 3 - 5 + 27 = 32
/// 4 - rad(4320) = 30 < 32
///```
/// It turns out that abc-hits are quite rare and there are only thirty-one abc-hits for `c < 1000`, with `∑ c = 12523`.
///
/// Find `∑ c` for `c < 120000`.
pub struct Solver127 {
    pub n: u64,
}

impl Default for Solver127 {
    fn default() -> Self {
        Self { n: 120_000 }
    }
}

impl Solver for Solver127 {
    fn problem_name(&self) -> &str { "acb-hits" }

    fn solve(&self) -> i64 {
        // (2..self.n).map(|c| (1..c / 2).filter(|&a| {
        //     let b = c - a;
        //     are_coprime(a.as_i64(), b.as_i64()) && radical((a * b * c).as_u64()) < c.as_u64()
        // }).count().as_u64() * c
        // ).sum::<u64>().as_i64()

        // let primes = primes_up_to(ceil_sqrt_u64(self.n)).collect::<Vec<_>>();
        // let product_radical = |a, b, c| {
        //     let mut factors = prime_factors_with_cache(a, &primes);
        //     factors.extend(prime_factors_with_cache(b, &primes));
        //     factors.extend(prime_factors_with_cache(c, &primes));
        //     factors.keys().product::<u64>()
        // };

        // let primes = primes_up_to(ceil_sqrt_u64(self.n)).collect::<Vec<_>>();
        // let f_cache = once(vec![]).chain((1..=self.n).map(|n| prime_factors_with_cache(n, &primes).into_keys().collect::<Vec<_>>())).collect::<Vec<_>>();
        // let product_radical = |a: u64, b: u64, c: u64| f_cache[a.as_usize()].iter().chain(f_cache[b.as_usize()].iter()).chain(f_cache[c.as_usize()].iter()).collect::<HashSet<_>>().into_iter().product::<u64>();

        // // since there are no common factors (GCD == 1) the radical of the product is equal to the product of the radicals
        // let radicals = once(1).chain((1..=self.n).map(|n| radical(n))).collect::<Vec<_>>();

        // // faster way to calculate radicals by sieving instead of multiplying the prime factors (from problem 124)
        // let mut radicals = vec![1; self.n.as_usize()];
        // (2..radicals.len()).for_each(|p| if radicals[p] == 1 { (p..self.n.as_usize()).step_by(p).for_each(|m| radicals[m] *= p.as_u64()) });
        // let product_radical = |a: u64, b: u64, c: u64| radicals[a.as_usize()] * radicals[b.as_usize()] * radicals[c.as_usize()];
        //
        // faray_coprimes(self.n).filter(|&(a, b, c)| product_radical(a, b, c) < c)
        //     .inspect(|&(a, b, c)| println!("{:4?} {:4?} {:4?} --- {:?}", a, b, c, product_radical(a, b, c)))
        //     .map(|(_, _, c)| c).sum::<u64>().as_i64()
 
        let radicals = radicals_up_to(self.n);

        // inverse of the radical function, ordered to iterate over increasing values of `a` and `radical[a]`
        let rad_list = radicals.iter().enumerate().skip(1).take_while(|&(value, _)| value.as_u64() < self.n / 2).fold(BTreeMap::<&u64, Vec<u64>>::new(), |mut acc, (value, radical)| {
            (*acc.entry(radical).or_default()).push(value.as_u64());
            acc
        });

        // for each `c` iterate over possible values of `a` (coprimes up to `c / rad(c)`) and count the values that produce a `a (c - a) c` hit
        (3..self.n).map(|c| (c, c / radicals[c.as_usize()])).map(|(c, a_ceil)| {
            rad_list.iter().take_while(|(&&rad_a, _)| rad_a < a_ceil).filter(|(&&rad_a, _)| are_coprime(rad_a, c)).map(|(&rad_a, a_values)| {
                a_values.iter().take_while(|&&a| a < c / 2).filter(|&&a| rad_a * radicals[(c - a).as_usize()] < a_ceil).count().as_u64()
            }).sum::<u64>() * c
        }).sum::<u64>().as_i64()
    }
}

fn _faray_coprimes(ceil: u64) -> impl Iterator<Item=(u64, u64, u64)> {
    let (mut a, mut b, mut c, mut d) = (0, 1, 1, ceil);
    from_fn(move || {
        while c <= ceil {
            let (factor, sum) = ((ceil + b) / d, c + d);
            (a, b, c, d) = (c, d, factor * c - a, factor * d - b);
            if sum < ceil {
                return Some((a, b, sum));
            }
        }
        None
    })
}
