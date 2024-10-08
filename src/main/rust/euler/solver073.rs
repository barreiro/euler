// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use crate::algorithm::cast::Cast;
use crate::Solver;

const LOWER: (u64, u64) = (1, 3);
const UPPER: (u64, u64) = (1, 2);

/// Consider the fraction, `n/d`, where `n` and `d` are positive integers. If `n<d` and `HCF(n,d)=1`, it is called a reduced proper fraction.
/// If we list the set of reduced proper fractions for `d ≤ 8` in ascending order of size, we get:
/// ```
/// 1/8, 1/7, 1/6, 1/5, 1/4, 2/7, 1/3, 3/8, 2/5, 3/7, 1/2, 4/7, 3/5, 5/8, 2/3, 5/7, 3/4, 4/5, 5/6, 6/7, 7/8
/// ```
/// It can be seen that there are `3` fractions between `1/3` and `1/2`.
/// How many fractions lie between `1/3` and `1/2` in the sorted set of reduced proper fractions for `d ≤ 12,000`?
pub struct Solver073 {
    pub n: usize,
}

impl Default for Solver073 {
    fn default() -> Self {
        Self { n: 12_000 }
    }
}

impl Solver for Solver073 {
    fn problem_name(&self) -> &str { "Counting fractions in a range" }

    fn solve(&self) -> i64 {
        // return numbers of irreducible fractions a/b < n/d where b is less than size
        // algorithm from "Computer Order Statistics in the Farey Sequence" by C. & M. Patrascu
        // http://people.csail.mit.edu/mip/papers/farey/talk.pdf
        let farey_rank = |(n, d), size| {
            let mut data = (0..=size as u64).map(|i| i * n / d).collect::<Vec<_>>();

            // remove all multiples of 2*i, 3*i, 4*i, ... similar to a prime sieve
            (1..=size / 2).for_each(|i| (2 * i..=size).step_by(i).for_each(|j| data[j] -= data[i]));
            data.iter().sum::<u64>().as_i64()
        };

        farey_rank(UPPER, self.n) - farey_rank(LOWER, self.n) - 1
    }
}
