// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::convert::TryFrom;

use algorithm::cast::Cast;
use algorithm::combinatorics::permutations_of_digits_with;
use algorithm::digits::concatenation;
use algorithm::long::arithmetic_sum_u64;
use Solver;

/// Consider the following "magic" `3-gon` ring, filled with the numbers `1` to `6`, and each line adding to nine.
///
/// `     +---+                   `
/// `     | 4 |                   `
/// `     +---+\                  `
/// `           \                 `
/// `          +---+              `
/// `          | 3 |              `
/// `         /+---+\             `
/// `        /       \            `
/// `     +---+     +---+    +---+`
/// `     | 1 |-----| 2 |----| 6 |`
/// `    /+---+     +---+    +---+`
/// `   /                         `
/// `+---+                        `
/// `| 5 |                        `
/// `+---+                        `
///
/// Working clockwise, and starting from the group of three with the numerically lowest external node (`4,3,2` in this example), each solution can be described uniquely.
/// For example, the above solution can be described by the set: `4,3,2`; `6,2,1`; `5,1,3`.
///
/// It is possible to complete the ring with four different totals: `9, 10, 11, and 12`. There are eight solutions in total.
///
/// `Total           Solution Set`
/// `    9    4,2,3; 5,3,1; 6,1,2`
/// `    9    4,3,2; 6,2,1; 5,1,3`
/// `   10    2,3,5; 4,5,1; 6,1,3`
/// `   10    2,5,3; 6,3,1; 4,1,5`
/// `   11    1,4,6; 3,6,2; 5,2,4`
/// `   11    1,6,4; 5,4,2; 3,2,6`
/// `   12    1,5,6; 2,6,4; 3,4,5`
/// `   12    1,6,5; 3,5,4; 2,4,6`
///
/// By concatenating each group it is possible to form `9-digit` strings; the maximum string for a `3-gon` ring is `432621513`.
///
/// Using the numbers `1` to `10`, and depending on arrangements, it is possible to form `16-` and `17-digit` strings.
/// What is the maximum `16-digit` string for a "magic" `5-gon` ring?
pub struct Solver068 {
    pub n: u64,
}

impl Default for Solver068 {
    fn default() -> Self {
        Self { n: 5 }
    }
}

impl Solver for Solver068 {
    fn solve(&self) -> i64 {
        // assumes there is an arrangement for the x-gon where all the smallest values are on the inside (true for odd x)
        let target_sum = (arithmetic_sum_u64(self.n * 2) + arithmetic_sum_u64(self.n)) / self.n;

        permutations_of_digits_with(1, u8::try_from(self.n).unwrap(), |inners| {
            (target_sum - u64::from(inners[0]) - u64::from(inners[1]) == self.n + 1).then(|| { // the first outer must be self.n + 1 for the solution to be 'canonical'
                let mut outers = inners.windows(2).map(|i| target_sum - u64::from(i[0]) - u64::from(i[1])).collect::<Vec<_>>();
                outers.push(target_sum - u64::from(inners[0]) - u64::from(inners[inners.len() - 1]));
                ((2..=self.n).all(|n| outers.contains(&(self.n + n)))).then(||
                    outers.iter().enumerate().flat_map(|(i, &o)| [o, u64::from(inners[i]), u64::from(inners[(i + 1) % inners.len()])]).collect::<Vec<_>>()
                )
            })
        }).max().unwrap().unwrap().into_iter().fold(0, concatenation).as_i64()
    }
}
