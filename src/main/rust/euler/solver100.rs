// COPYRIGHT (C) 2022 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::long::{pow_10, square};
use euler::Solver;

// If a box contains twenty-one coloured discs, composed of fifteen blue discs and six red discs, and two discs were taken at random, it can be seen that the probability of taking two blue discs, P(BB) = (15/21)×(14/20) = 1/2.
//
// The next such arrangement, for which there is exactly 50% chance of taking two blue discs at random, is a box containing eighty-five blue discs and thirty-five red discs.
//
// By finding the first arrangement to contain over 10^12 = 1,000,000,000,000 discs in total, determine the number of blue discs that the box would contain.

pub struct Solver100 {
    pub n: isize,
}

impl Default for Solver100 {
    fn default() -> Self {
        Solver100 { n: 12 }
    }
}

impl Solver for Solver100 {
    fn solve(&self) -> isize {
        // inefficient and would have to go over the limit for 64 bit integer
        // for blues in 2..pow_10(self.n.min(7)) {
        //     let total_b = (blues * (blues - 1)) << 1;
        //     let floor_d = floor_sqrt(total_b);
        //     // if is_square((total_b - floor_d) as isize) {
        //     if total_b % floor_d == 0 && total_b % (floor_d + 1) == 0 {
        //         println!("### {:15?} w/ {:?} blues", floor_d + 1, blues);
        //     }
        // }

        // see https://en.wikipedia.org/wiki/Pell_number#Square_triangular_numbers
        // look at the table. the number of red discs is `s` and the total number of discs is `b`
        pell_numbers().scan(1, |state, pell| {
            // let discs = ((*state * pell) << 1) + if is_odd(pell) { 1 } else { 0 };

            // pell numbers can generate pythagorean triples where the legs are apart by one `t^2 + (t - 1)^2 = s^2`
            let discs = isize::max((*state * pell) << 1, square(pell) - square(*state));
            let blues = discs - *state * (pell - *state);
            *state = pell;
            Some((discs, blues))
        }).find_map(|(discs, blues)| (discs > pow_10(self.n)).then_some(blues)).unwrap()
    }
}

// --- //

fn pell_numbers() -> impl Iterator<Item=isize> {
    PellIterator { a: 0, b: 1 }
}

struct PellIterator {
    a: isize,
    b: isize,
}

impl Iterator for PellIterator {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        let next = 2 * self.b + self.a;
        self.a = self.b;
        self.b = next;
        Some(next)
    }
}

