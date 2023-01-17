// COPYRIGHT (C) 2022 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::iter::from_fn;

use algorithm::cast::Cast;
use algorithm::root::{pow_10, square_u64};
use Solver;

/// If a box contains twenty-one coloured discs, composed of fifteen blue discs and six red discs, and two discs were taken at random, it can be seen that the probability of taking two blue discs, `P(BB) = (15/21)×(14/20) = 1/2`.
///
/// The next such arrangement, for which there is exactly `50%` chance of taking two blue discs at random, is a box containing eighty-five blue discs and thirty-five red discs.
///
/// By finding the first arrangement to contain over `10^12 = 1,000,000,000,000` discs in total, determine the number of blue discs that the box would contain.
pub struct Solver100 {
    pub n: u64,
}

impl Default for Solver100 {
    fn default() -> Self {
        Self { n: 12 }
    }
}

impl Solver for Solver100 {
    fn solve(&self) -> i64 {
        // see https://en.wikipedia.org/wiki/Pell_number#Square_triangular_numbers
        // look at the table. the number of red discs is `s` and the total number of discs is `b`
        pell_numbers().scan(1, |state, pell| {
            // pell numbers can generate pythagorean triples where the legs are apart by one `t^2 + (t - 1)^2 = s^2`
            let discs = u64::max((*state * pell) * 2, square_u64(pell) - square_u64(*state));
            let blues = discs - *state * (pell - *state);
            *state = pell;
            Some((discs, blues))
        }).find_map(|(discs, blues)| (discs > pow_10(self.n)).then_some(blues)).as_i64()
    }
}

// --- //

fn pell_numbers() -> impl Iterator<Item=u64> {
    let (mut a, mut b) = (0, 1);
    from_fn(move || {
        let next = 2 * b + a;
        a = b;
        b = next;
        Some(next)
    })
}
