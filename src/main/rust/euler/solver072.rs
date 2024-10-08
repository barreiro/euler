// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use crate::algorithm::cast::Cast;
use crate::Solver;

/// Consider the fraction, n/d, where n and d are positive integers. If `n<d` and `HCF(n,d)=1`, it is called a reduced proper fraction.
/// If we list the set of reduced proper fractions for `d ≤ 8` in ascending order of size, we get:
/// ```
/// 1/8, 1/7, 1/6, 1/5, 1/4, 2/7, 1/3, 3/8, 2/5, 3/7, 1/2, 4/7, 3/5, 5/8, 2/3, 5/7, 3/4, 4/5, 5/6, 6/7, 7/8
/// ```
/// It can be seen that there are `21` elements in this set.
/// How many elements would be contained in the set of reduced proper fractions for `d ≤ 1,000,000`?
pub struct Solver072 {
    pub n: usize,
}

impl Default for Solver072 {
    fn default() -> Self {
        Self { n: 1_000_000 }
    }
}

impl Solver for Solver072 {
    fn problem_name(&self) -> &str { "Counting fractions" }

    fn solve(&self) -> i64 {
        // calculate the number of fractions in the Farey sequence using a (recursive) Mobius inversion formula, and remove 0/1 and 1/1
        farey_size(self.n, &mut vec![0; self.n + 1]).as_i64() - 2
    }
}

fn farey_size(n: usize, cache: &mut [usize]) -> usize {
    let recursion = |m| if cache[m] == 0 { farey_size(m, cache) } else { cache[m] };
    let value = (((n + 3) * n) / 2) - (2..=n).map(|d| n / d).map(recursion).sum::<usize>();
    cache[n] = value;
    value
}
