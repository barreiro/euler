// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::cast::Cast;
use algorithm::continued_fraction::add_mul;
use algorithm::prime::primes_up_to;
use algorithm::root::{floor_sqrt_u64, pow_10};
use Solver;

/// Consider quadratic Diophantine equations of the form:
///
/// `x ^ 2 – D * y ^ 2 = 1`
///
/// For example, when `D = 13`, the minimal solution in `x` is `649 ^ 2 – 13 * 180 ^ 2 = 1`.
///
/// It can be assumed that there are no solutions in positive integers when `D` is square.
///
/// By finding minimal solutions in `x` for `D = {2, 3, 5, 6, 7}`, we obtain the following:
///
/// `3 ^ 2 – 2 * 2 ^ 2 = 1`
/// `2 ^ 2 – 3 * 1 ^ 2 = 1`
/// `9 ^ 2 – 5 * 4 ^ 2 = 1`
/// `5 ^ 2 – 6 * 2 ^ 2 = 1`
/// `8 ^ 2 – 7 * 3 ^ 2 = 1`
///
/// Hence, by considering minimal solutions in `x` for `D ≤ 7`, the largest `x` is obtained when `D = 5`.
///
/// Find the value of `D ≤ 1000` in minimal solutions of `x` for which the largest value of `x` is obtained.
pub struct Solver066 {
    pub n: u64,
}

impl Default for Solver066 {
    fn default() -> Self {
        Self { n: 1000 }
    }
}

impl Solver for Solver066 {
    fn solve(&self) -> i64 {
        let custom_comparator = |n: Vec<_>| (n.len(), n[n.len() - 1]);
        let (cache_sqrt, threshold) = ((0..self.n).map(floor_sqrt_u64).collect::<Vec<_>>(), pow_10(15));

        // algorithm is described in the paper "An Algorithm to Solve a Pell Equation" by Alexander Junod
        // https://www.emis.de/journals/GMN/yahoo_site_admin/assets/docs/1_GMN-8492-V28N2.190180001.pdf
        let junod = |d: u64| {
            let (mut previous, mut current, root) = ((vec![0], 1, d), (vec![1], 0, 1), cache_sqrt[d.as_usize()]);
            while current.2 > 2 || current.1 == 0 {
                let (mut a_p, _, c_p) = previous;
                let (a, b, c) = current;
                let p_factor = cache_sqrt[(d - c * c_p).as_usize()];
                let q = (p_factor + root) / c;

                add_mul(&mut a_p, &a, q, threshold);
                previous = (a, b, c);
                current = (a_p, 1, c_p + 2 * q * p_factor - q * q * c); // ( q * a + a_p,  q * b + b_p, c_next)
            }
            if current.2 == 1 { // c is either 1 or 2, normalize by multiplying by 2
                add_mul(&mut current.0, &[0], 2, threshold);
            }
            current.0 // outputs 2 * a / c instead of x ( actual value of x = ±1 + (2 * a^2) / c )
        };

        // optimization: only check prime numbers because these generate the biggest numerators
        primes_up_to(self.n).max_by_key(|&d| custom_comparator(junod(d))).as_i64()
    }
}
