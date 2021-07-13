// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::combinatorics::permutations_of_set_with;
use algorithm::long::{pow_10, square, to_digits};
use euler::algorithm::combinatorics::combinations_with;
use euler::Solver;
use std::collections::HashSet;

// Each of the six faces on a cube has a different digit (0 to 9) written on it; the same is done to a second cube.
// By placing the two cubes side-by-side in different positions we can form a variety of 2-digit numbers.
//
// For example, the square number 64 could be formed: 6 4
//
// In fact, by carefully choosing the digits on both cubes it is possible to display all of the square numbers below one-hundred: 01, 04, 09, 16, 25, 36, 49, 64, and 81.
//
// For example, one way this can be achieved is by placing {0, 5, 6, 7, 8, 9} on one cube and {1, 2, 3, 4, 8, 9} on the other cube.
//
// However, for this problem we shall allow the 6 or 9 to be turned upside-down so that an arrangement like {0, 5, 6, 7, 8, 9} and {1, 2, 3, 4, 6, 7} allows for all nine square numbers to be displayed; otherwise it would be impossible to obtain 09.
//
// In determining a distinct arrangement we are interested in the digits on each cube, not the order.
//
// {1, 2, 3, 4, 5, 6} is equivalent to {3, 6, 4, 1, 2, 5}
// {1, 2, 3, 4, 5, 6} is distinct from {1, 2, 3, 4, 5, 9}
//
// But because we are allowing 6 and 9 to be reversed, the two distinct sets in the last example both represent the extended set {1, 2, 3, 4, 5, 6, 9} for the purpose of forming 2-digit numbers.
//
// How many distinct arrangements of the two cubes allow for all of the square numbers to be displayed?

const DICE_SIZE: usize = 6;

pub struct Solver090 {
    pub n: isize,
}

impl Default for Solver090 {
    fn default() -> Self {
        Solver090 { n: 2 }
    }
}

impl Solver for Solver090 {
    fn solve(&self) -> isize {
        // creates a list of all permutations of the squares, replacing 9 with 6
        let squares = (1..).map(square).take_while(|&s| s < pow_10(self.n)).map(to_digits).map(|digits| {
            let mut s = digits.iter().map(|&d| if d == 9 { 6 } else { d }).collect::<Vec<_>>();
            s.resize(self.n as _, 0);
            s.sort_unstable(); // needs to be sorted for permutations_of_set_with()
            s
        }).map(|s| permutations_of_set_with(s.to_vec(), |p| Some(p.to_vec())).collect::<Vec<_>>()).collect::<HashSet<_>>();

        // create all combinations from digits to form the dice, replacing 9 with 6.
        let dice = combinations_with((0..=9).map(|x| if x == 9 { 6 } else { x }).collect(), DICE_SIZE, |c| Some(c.to_vec())).collect::<Vec<_>>();

        // cartesian picks all combinations of n dice and for each combination verify if all the squares are present.
        cartesian_with(dice.len(), self.n as _, |dice_index| squares.iter().all(|sq| sq.iter().any(|p| (0..p.len()).all(|i| dice[dice_index[i]].contains(&p[i]))))).count() as _
    }
}

// --- //

struct Cartesian<F> where F: Fn(&[usize]) -> bool {
    indexes: Vec<usize>,
    predicate: F,
}

/// creates an iterator for combinations of n dimensions up to a certain value
fn cartesian_with<F>(value: usize, dimensions: usize, predicate: F) -> Cartesian<F> where F: Fn(&[usize]) -> bool {
    Cartesian { indexes: vec![value - 1; dimensions], predicate }
}

impl<F> Iterator for Cartesian<F> where F: Fn(&[usize]) -> bool {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            for i in (0..self.indexes.len()).rev() {
                if self.indexes[i] != 0 {
                    self.indexes[i] -= 1;
                    if self.indexes[0] == 0 {
                        return None;
                    } else if (self.predicate)(&self.indexes) {
                        return Some(true);
                    } else {
                        break;
                    }
                } else {
                    (i..self.indexes.len()).for_each(|j| if self.indexes[i - 1] != 0 { self.indexes[j] = self.indexes[i - 1] - 1 });
                }
            }
        }
    }
}

