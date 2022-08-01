// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::combinatorics::permutations_with;
use euler::algorithm::long::{arithmetic_sum, concatenation};
use euler::Solver;

// Consider the following "magic" 3-gon ring, filled with the numbers 1 to 6, and each line adding to nine.
//
//      +---+
//      | 4 |
//      +---+\
//            \
//           +---+
//           | 3 |
//          /+---+\
//         /       \
//      +---+     +---+    +---+
//      | 1 |-----| 2 |----| 6 |
//     /+---+     +---+    +---+
//    /
// +---+
// | 5 |
// +---+
//
// Working clockwise, and starting from the group of three with the numerically lowest external node (4,3,2 in this example), each solution can be described uniquely.
// For example, the above solution can be described by the set: 4,3,2; 6,2,1; 5,1,3.
//
// It is possible to complete the ring with four different totals: 9, 10, 11, and 12. There are eight solutions in total.
//
// Total           Solution Set
//     9    4,2,3; 5,3,1; 6,1,2
//     9    4,3,2; 6,2,1; 5,1,3
//    10    2,3,5; 4,5,1; 6,1,3
//    10    2,5,3; 6,3,1; 4,1,5
//    11    1,4,6; 3,6,2; 5,2,4
//    11    1,6,4; 5,4,2; 3,2,6
//    12    1,5,6; 2,6,4; 3,4,5
//    12    1,6,5; 3,5,4; 2,4,6
//
// By concatenating each group it is possible to form 9-digit strings; the maximum string for a 3-gon ring is 432621513.
//
// Using the numbers 1 to 10, and depending on arrangements, it is possible to form 16- and 17-digit strings.
// What is the maximum 16-digit string for a "magic" 5-gon ring?

pub struct Solver068 {
    pub n: isize,
}

impl Default for Solver068 {
    fn default() -> Self {
        Solver068 { n: 5 }
    }
}

impl Solver for Solver068 {
    fn solve(&self) -> isize {
        // assumes there is an arrangement for the x-gon where all the smallest values are on the inside (true for odd x)
        let target_sum = (arithmetic_sum(2 * self.n) + arithmetic_sum(self.n)) / self.n;

        permutations_with(1, self.n, |inners| {
            if target_sum - inners[0] - inners[1] == self.n + 1 { // the first outer must be self.n + 1 for the solution to be 'canonical'
                let mut outers = inners.windows(2).map(|i| target_sum - i[0] - i[1]).collect::<Vec<_>>();
                outers.push(target_sum - inners[0] - inners[inners.len() - 1]);
                if (2..=self.n).all(|n| outers.contains(&(self.n + n))) {
                    return Some(outers.iter().enumerate().flat_map(|(i, &o)| [o, inners[i], inners[(i + 1) % inners.len()]]).collect::<Vec<_>>())
                }
            }
            None
        }).max().unwrap().iter().fold(0, |c, &d| concatenation(c, d))
    }
}
