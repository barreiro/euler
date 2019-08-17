// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::collections::HashMap;

use euler::algorithm::long::{concatenation, from_digits, pow_10, to_digits};
use euler::algorithm::prime::generator_trial_division;
use euler::Solver;

// The arithmetic sequence, 1487, 4817, 8147, in which each of the terms increases by 3330, is unusual in two ways:
// (i) each of the three terms are prime, and, (ii) each of the 4-digit numbers are permutations of one another.
//
// There are no arithmetic sequences made up of three 1-, 2-, or 3-digit primes, exhibiting this property, but there is one other 4-digit increasing sequence.
// What 12-digit number do you form by concatenating the three terms in this sequence?

const SEQ: usize = 1;

pub struct Solver049 {
    pub n: isize
}

impl Default for Solver049 {
    fn default() -> Self {
        Solver049 { n: 4 }
    }
}

impl Solver for Solver049 {
    fn solve(&self) -> isize {
        // group together primes based on their permutation --- using their sorted digits as the key on a map
        let mut grouped_primes = HashMap::with_capacity(pow_10(self.n) as _);
        generator_trial_division().skip_while(|&p| p < pow_10(self.n - 1)).take_while(|&p| p < pow_10(self.n)).for_each(|d| {
            let mut digits = to_digits(d);
            if !digits.contains(&0) {
                digits.sort_unstable();
                grouped_primes.entry(from_digits(digits)).or_insert(Vec::new()).push(d);
            }
        });
        let mut permutations = grouped_primes.values().filter(|&p| p.len() >= 3).collect::<Vec<_>>();
        permutations.sort_unstable();

        // given a list of primes, finds if there is one that is in between two others
        let condition = |&p: &&Vec<isize>| {
            for i in 0..p.len() - 2 {
                for j in i + 2..p.len() {
                    if p.contains(&(p[j] + p[i] >> 1)) {
                        return Some([p[i], (p[j] + p[i]) >> 1, p[j]]);
                    }
                }
            }
            None
        };

        let sequence = permutations.iter().filter_map(condition).nth(SEQ).unwrap();
        from_digits(sequence.iter().fold(Vec::new(), |acc, &p| concatenation(&acc, &to_digits(p))))
    }
}
