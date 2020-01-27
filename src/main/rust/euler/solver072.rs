// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::Solver;

// Consider the fraction, n/d, where n and d are positive integers. If n<d and HCF(n,d)=1, it is called a reduced proper fraction.
// If we list the set of reduced proper fractions for d ≤ 8 in ascending order of size, we get:
//
// 1/8, 1/7, 1/6, 1/5, 1/4, 2/7, 1/3, 3/8, 2/5, 3/7, 1/2, 4/7, 3/5, 5/8, 2/3, 5/7, 3/4, 4/5, 5/6, 6/7, 7/8
//
// It can be seen that there are 21 elements in this set.
// How many elements would be contained in the set of reduced proper fractions for d ≤ 1,000,000?

pub struct Solver072 {
    pub n: isize
}

impl Default for Solver072 {
    fn default() -> Self {
        Solver072 { n: 1_000_000 }
    }
}

impl Solver for Solver072 {
    fn solve(&self) -> isize {
        // initial approach, to sum the totients up to self.n:
//         (2..=self.n).map(totient).sum()

        // improve  by calculating totients iteratively
//        let mut tot = (0..=self.n).collect::<Vec<_>>();
//        (2..=self.n).for_each(|i| if tot[i as usize] == i {
//            (i..=self.n).step_by(i as _).for_each(|j| tot[j as usize] = tot[j as usize] * (i - 1) / i);
//        });
//        tot[2..].iter().sum::<isize>()

        // calculate the rank of 1 (see problem 073)
//        f_rank((1, 1), self.n) - 1

        // calculate the number of fractions in the Farey sequence using a (recursive) Mobius inversion formula, and remove 0/1 ans 1/1
        farey_size(self.n as _, &mut vec![0; self.n as usize + 1]) as isize - 2
    }
}

pub fn farey_size(n: usize, cache: &mut [usize]) -> usize {
    let recursion = |m| if cache[m] != 0 { cache[m] } else { farey_size(m, cache) };
    let value = ((n + 3) * n >> 1) - (2..=n).map(|d| n / d).map(recursion).sum::<usize>();
    cache[n] = value;
    value
}