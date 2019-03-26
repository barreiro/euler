// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::Solver;

// Each new term in the Fibonacci sequence is generated by adding the previous two terms. By starting with 1 and 2, the first 10 terms will be:
// 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
// By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.

pub struct Solver002 {
    pub n: isize
}

impl Default for Solver002 {
    fn default() -> Self {
        Solver002 { n: 4000000 }
    }
}

impl Solver for Solver002 {
    fn solve(&self) -> isize {
        let (mut previous, mut last, mut sum) = (1, 2, 2);
        while last < self.n {
            // previous moves to two terms ahead of last. last gets the next after the new previous. it's even by definition!
            previous = previous + last + last;
            last = previous + previous - last;
            sum += last
        }
        sum - last
    }
}
