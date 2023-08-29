// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::mem::swap;

use algorithm::cast::Cast;
use algorithm::continued_fraction::add_mul;
use algorithm::digits::digits_sum;
use algorithm::root::pow_10;
use Solver;

/// The square root of `2` can be written as an infinite continued fraction.
/// ```
///`√2 = 1 + 1 / ( 2 + ( 1 / 2 + ( 1 / ( 2 + ( 1 / 2 + ...
/// ```
/// The infinite continued fraction can be written, `√2 = [ 1; (2) ]`, `(2)` indicates that `2` repeats ad infinitum. In a similar way, `√23 = [ 4; ( 1, 3, 1, 8 ) ]`.
///
/// It turns out that the sequence of partial values of continued fractions for square roots provide the best rational approximations. Let us consider the convergents for `√2`.
/// ```
/// 1 + 1 / 2 = 3 / 2
/// 1 + 1 / ( 2 + ( 1 / 2 ) ) = 7 / 5
/// 1 + 1 / { 2 + ( 1 / ( 2 + ( 1 / 2 ) ) ) ) = 17 / 12
/// 1 + 1 / ( 2 + ( 1 / ( 2 + ( 1 / ( 2 + ( 1 / 2 ) ) ) ) ) ) = 41 / 29
/// ```
/// Hence the sequence of the first ten convergents for `√2` are:
/// `1, 3/2, 7/5, 17/12, 41/29, 99/70, 239/169, 577/408, 1393/985, 3363/2378, ...`
///
/// What is most surprising is that the important mathematical constant,
/// `e = [ 2; 1, 2, 1, 1, 4, 1, 1, 6, 1, ..., 1, 2k, 1, ... ]`.
///
/// The first ten terms in the sequence of convergents for e are:
/// `2, 3, 8/3, 11/4, 19/7, 87/32, 106/39, 193/71, 1264/465, 1457/536, ...`
///
/// The sum of digits in the numerator of the 10th convergent is `1 + 4 + 5 + 7 = 17`.
/// Find the sum of digits in the numerator of the `100th` convergent of the continued fraction for `e`.
pub struct Solver065 {
    pub n: usize,
}

impl Default for Solver065 {
    fn default() -> Self {
        Self { n: 100 }
    }
}

impl Solver for Solver065 {
    fn problem_name(&self) -> &str { "Convergents of e" }

    fn solve(&self) -> i64 {
        // optimization of convergent_with that only calculates numerators
        let convergent_numerator = |f: fn(_) -> _, nth| {
            let (mut n, mut d, threshold) = (vec![f(nth - 1)], vec![1], pow_10(15));
            (0..nth - 1).rev().for_each(|i| {
                swap(&mut d, &mut n);
                add_mul(&mut n, &d, f(i), threshold);
            });
            n
        };

        let e_expansion = |n: usize| if n == 0 { 2 } else if n % 3 == 2 { (n.as_u64() + 2) * 2 / 3 } else { 1 };
        convergent_numerator(e_expansion, self.n).into_iter().map(digits_sum).sum()
    }
}
