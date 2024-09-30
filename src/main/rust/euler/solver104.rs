// COPYRIGHT (C) 2022 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::iter::from_fn;

use crate::algorithm::cast::Cast;
use crate::algorithm::filter::is_pandigital;
use crate::algorithm::root::pow_10_usize;
use crate::Solver;

/// The Fibonacci sequence is defined by the recurrence relation: `Fn = Fn−1 + Fn−2`, where `F1 = 1` and `F2 = 1`.
///
/// It turns out that `F541`, which contains `113` digits, is the first Fibonacci number for which the last nine digits are 1-9 pandigital (contain all the digits 1 to 9, but not necessarily in order). And `F2749`, which contains `575` digits, is the first Fibonacci number for which the first nine digits are 1-9 pandigital.
///
/// Given that `Fk` is the first Fibonacci number for which the first nine digits AND the last nine digits are 1-9 pandigital, find `k`.
pub struct Solver104 {
    pub n: usize,
}

impl Default for Solver104 {
    fn default() -> Self {
        Self { n: 9 }
    }
}

impl Solver for Solver104 {
    fn problem_name(&self) -> &str { "Pandigital fibonacci ends" }

    fn solve(&self) -> i64 {
        let both_pandigital = |n, tail| is_pandigital(&tail) && is_pandigital(&head_binet(n, self.n));
        (2..).zip(fibonacci_tail(self.n)).find_map(|(n, tail)| both_pandigital(n, tail).then_some(n)).as_i64()
    }
}

// --- //

// from https://r-knott.surrey.ac.uk/Fibonacci/fibFormula.html#section2.4
// using the fractional part of the logarithm (base 10) of Binet's Formula (approximation)
#[allow(clippy::cast_possible_truncation)]
fn head_binet(n: usize, len: usize) -> u64 {
    let (log_root_5, log_phi) = (5_f64.log10() / 2.0, ((1.0 + 5_f64.sqrt()) / 2.0).log10());
    10.0_f64.powf(f64::from((len - 1) as u32) + f64::from(n as u32).mul_add(log_phi, -log_root_5).fract()).as_u64()
}

fn fibonacci_tail(len: usize) -> impl Iterator<Item=u64> {
    let (mut a, mut b, threshold) = (0, 1, pow_10_usize(len));
    from_fn(move || {
        let mut next = a + b;
        if next >= threshold { next -= threshold }
        (a, b) = (b, next);
        Some(next)
    })
}
