// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::collections::HashSet;
use std::iter::from_fn;

use crate::algorithm::cast::Cast;
use crate::algorithm::combinatorics::{combinations, permutations_of_set};
use crate::algorithm::digits::digits_iter;
use crate::algorithm::filter::less_than_u64;
use crate::algorithm::root::{pow_10_usize, square_u64};
use crate::Solver;

const DICE_SIZE: usize = 6;

/// Each of the six faces on a cube has a different digit (`0` to `9`) written on it; the same is done to a second cube.
/// By placing the two cubes side-by-side in different positions we can form a variety of `2-digit` numbers.
///
/// For example, the square number `64` could be formed: `6` `4`
///
/// In fact, by carefully choosing the digits on both cubes it is possible to display all the square numbers below one-hundred: `01, 04, 09, 16, 25, 36, 49, 64, and 81`.
///
/// For example, one way this can be achieved is by placing `{0, 5, 6, 7, 8, 9}` on one cube and `{1, 2, 3, 4, 8, 9}` on the other cube.
///
/// However, for this problem we shall allow the `6` or `9` to be turned upside-down so that an arrangement like `{0, 5, 6, 7, 8, 9}` and `{1, 2, 3, 4, 6, 7}` allows for all nine square numbers to be displayed; otherwise it would be impossible to obtain `09`.
///
/// In determining a distinct arrangement we are interested in the digits on each cube, not the order.
///
/// `{1, 2, 3, 4, 5, 6}` is equivalent to `{3, 6, 4, 1, 2, 5}`
/// `{1, 2, 3, 4, 5, 6}` is distinct from `{1, 2, 3, 4, 5, 9}`
///
/// But because we are allowing `6` and `9` to be reversed, the two distinct sets in the last example both represent the extended set `{1, 2, 3, 4, 5, 6, 9}` for the purpose of forming `2-digit` numbers.
///
/// How many distinct arrangements of the two cubes allow for all the square numbers to be displayed?
pub struct Solver090 {
    pub n: usize,
}

impl Default for Solver090 {
    fn default() -> Self {
        Self { n: 2 }
    }
}

impl Solver for Solver090 {
    fn problem_name(&self) -> &str { "Cube digit pairs" }

    #[allow(clippy::maybe_infinite_iter)]
    fn solve(&self) -> i64 {
        // creates a list of all permutations of the squares, replacing `9` with `6`
        let squares = (1..).map(square_u64).take_while(less_than_u64(pow_10_usize(self.n))).map(|n| {
            let mut s = digits_iter(n).map(|d| if d == 9 { 6 } else { d }).collect::<Vec<_>>();
            s.resize(self.n, 0);
            s.sort_unstable(); // needs to be sorted for `permutations_of_set()`
            s
        }).map(|s| permutations_of_set(s).collect::<Vec<_>>()).collect::<HashSet<_>>();

        // create all combinations from digits to form the dice, replacing `9` with `6`
        let dice = combinations((0..=9).map(|x| if x == 9 { 6 } else { x }).collect(), DICE_SIZE).collect::<Vec<_>>();

        // cartesian picks all combinations of n dice and for each combination verify if all the squares are present
        cartesian_with(dice.len(), self.n, |dice_index| squares.iter().all(|sq| sq.iter().any(|p| (0..p.len()).all(|i| dice[dice_index[i]].contains(&p[i]))))).count().as_i64()
    }
}

// --- //

/// creates an iterator for combinations of n dimensions up to a certain value
fn cartesian_with<F>(value: usize, dimensions: usize, predicate: F) -> impl Iterator<Item=bool> where F: Fn(&[usize]) -> bool {
    let mut indexes = vec![value - 1; dimensions];
    from_fn(move || loop {
        for i in (0..indexes.len()).rev() {
            if indexes[i] != 0 {
                indexes[i] -= 1;
                if indexes[0] == 0 {
                    return None;
                } else if predicate(&indexes) {
                    return Some(true);
                }
                break;
            }
            (i..indexes.len()).for_each(|j| if indexes[i - 1] != 0 { indexes[j] = indexes[i - 1] - 1 });
        }
    })
}

