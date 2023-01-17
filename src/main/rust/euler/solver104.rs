// COPYRIGHT (C) 2022 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::iter::from_fn;

use algorithm::cast::Cast;
use algorithm::filter::is_pandigital;
use algorithm::root::pow_10_u;
use Solver;

const _THRESHOLD: u64 = pow_10_u(18);

/// The Fibonacci sequence is defined by the recurrence relation:
///
/// `Fn = Fn−1 + Fn−2`, where `F1 = 1` and `F2 = 1`.
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
    fn solve(&self) -> i64 {
        // let _both_pandigital = |term, (head, tail)| is_pandigital(&last_digits(tail, self.n)) &&is_pandigital(&first_digits(head, self.n));
        // _fibonacci_head().zip(_fibonacci_tail()).enumerate().skip_while(|&(_, (_, tail))| tail < pow_10_u(self.n)).find_map(|(n, f)| _both_pandigital(n+2, f).then_some(n + 2)).unwrap().as_i64()

        // let mut p_cache = vec![false; pow_10_u(self.n).as_usize()];
        // permutations_of_digits_with(1, self.n as u8, |p| Some(Digits::from(p).value().as_usize()) ).for_each(|p| p_cache[p] = true );
        // let both_pandigital = |term, tail : u64| p_cache[tail.as_usize()] && p_cache[head_binet(term, self.n).as_usize()];

        // let _both_pandigital = |(head, tail)| is_pandigital(&tail) &&is_pandigital(&first_digits(head, self.n));
        // _fibonacci_head().zip(fibonacci_tail(pow_10_u(self.n))).enumerate().skip_while(|&(_, (_, tail))| tail < pow_10_u(self.n)).find_map(|(n, f)| _both_pandigital(f).then_some(n + 2)).unwrap().as_i64()

        let both_pandigital = |term, tail| is_pandigital(&tail) && is_pandigital(&head_binet(term, self.n));
        // fibonacci_tail(pow_10_u(self.n)).enumerate().find_map(|(n, tail)| both_pandigital(n + 2, tail).then_some(n + 2)).as_i64()
        (2..).zip(fibonacci_tail(pow_10_u(self.n))).find_map(|(n, tail)| both_pandigital(n, tail).then_some(n)).as_i64()
    }
}

// --- //

// from https://r-knott.surrey.ac.uk/Fibonacci/fibFormula.html#section2.4
// using the fractional part of the logarithm (base 10) of Binet's Formula (approximation)
#[allow(clippy::cast_possible_truncation)]
fn head_binet(term: usize, len: usize) -> u64 {
    let (log_root_5, log_phi) = (5_f64.log10() / 2.0, ((1.0 + 5_f64.sqrt()) / 2.0).log10());
    10.0_f64.powf(f64::from((len - 1) as u32) + (f64::from(term as u32) * log_phi - log_root_5).fract()).as_u64()
}

fn _fibonacci_head() -> impl Iterator<Item=u64> {
    let (mut a, mut b) = (0, 1);
    from_fn(move || {
        let next = a + b;
        (a, b) = if next >= _THRESHOLD { (b / 10, next / 10) } else { (b, next) };
        Some(next)
    })
}

fn _fibonacci_tail() -> impl Iterator<Item=u64> {
    let (mut a, mut b) = (0, 1);
    from_fn(move || {
        let next = (a + b) % _THRESHOLD;
        (a, b) = (b, next);
        Some(next)
    })
}

fn fibonacci_tail(modulo: u64) -> impl Iterator<Item=u64> {
    let (mut a, mut b) = (0, 1);
    from_fn(move || {
        let mut next = a + b;
        if next >= modulo { next -= modulo }
        (a, b) = (b, next);
        Some(next)
    })
}

