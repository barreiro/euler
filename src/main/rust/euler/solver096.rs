// COPYRIGHT (C) 2022 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::convert::TryFrom;

use crate::algorithm::cast::{char_as_i64, to_i64};
use crate::algorithm::digits::{Digit, Digits};
use crate::algorithm::io::load_default_data;
use crate::Solver;

const SUDOKU_SIZE: usize = 9;
const SUDOKU_SQUARE_SIZE: usize = 3; // floor_sqrt_u64(SUDOKU_SIZE as u64) as usize;

/// Su Doku (Japanese meaning number place) is the name given to a popular puzzle concept.
/// Its origin is unclear, but credit must be attributed to Leonhard Euler who invented a similar, and much more difficult, puzzle idea called Latin Squares.
/// The objective of Su Doku puzzles, however, is to replace the blanks (or zeros) in a `9 by 9` grid in such that each row, column, and `3 by 3` box contains each of the digits `1 to 9`.
///
/// Below is an example of a typical starting puzzle grid and its solution grid.
/// ```
/// 0 0 3 | 0 2 0 | 6 0 0   4 8 3 | 9 2 1 | 6 5 7
/// 9 0 0 | 3 0 5 | 0 0 1   9 6 7 | 3 4 5 | 8 2 1
/// 0 0 1 | 8 0 6 | 4 0 0   2 5 1 | 8 7 6 | 4 9 3
/// ------+-------+------   ------+-------+------
/// 0 0 8 | 1 0 2 | 9 0 0   5 4 8 | 1 3 2 | 9 7 6
/// 7 0 0 | 0 0 0 | 0 0 8   7 2 9 | 5 6 4 | 1 3 8
/// 0 0 6 | 7 0 8 | 2 0 0   1 3 6 | 7 9 8 | 2 4 5
/// ------+-------+------   ------+-------+------
/// 0 0 2 | 6 0 9 | 5 0 0   3 7 2 | 6 8 9 | 5 1 4
/// 8 0 0 | 2 0 3 | 0 0 9   8 1 4 | 2 5 3 | 7 6 9
/// 0 0 5 | 0 1 0 | 3 0 0   6 9 5 | 4 1 7 | 3 8 2
/// ```
/// A well constructed Su Doku puzzle has a unique solution and can be solved by logic, although it may be necessary to employ "guess and test" methods in order to eliminate options (there is much contested opinion over this).
/// The complexity of the search determines the difficulty of the puzzle; the example above is considered easy because it can be solved by straight forward direct deduction.
///
/// The 6K text file, `sudoku.txt` (right click and 'Save Link/Target As...'), contains fifty different Su Doku puzzles ranging in difficulty, but all with unique solutions (the first puzzle in the file is the example above).
///
/// By solving all fifty puzzles find the sum of the `3-digit` numbers found in the top left corner of each solution grid; for example, `483` is the `3-digit` number found in the top left corner of the solution grid above.
pub struct Solver096 {
    pub n: usize,
    pub input: String,
}

impl Default for Solver096 {
    fn default() -> Self {
        Self { n: 50, input: load_default_data(96) }
    }
}

impl Solver for Solver096 {
    fn problem_name(&self) -> &str { "Su Doku" }

    fn solve(&self) -> i64 {
        self.input.lines().map(|line| if line.starts_with("Grid") { "\n" } else { line }).collect::<String>().split_whitespace().take(self.n).map(|sudoku_str| {
            let mut sudoku = Sudoku::from(sudoku_str);
            sudoku.solve();

            sudoku.values().iter().take(SUDOKU_SQUARE_SIZE).rev().copied().collect::<Digits>().value()
        }).map(to_i64).sum()
    }
}

// --- //

// holds a Digit or a set of candidates encoded in an u64
#[derive(Clone)]
struct SudokuDigit {
    value: Digit,
    candidates: u64,
}

impl From<Digit> for SudokuDigit {
    fn from(value: Digit) -> Self {
        Self { value, candidates: if value == 0 { (2 << SUDOKU_SIZE) - 2 } else { 0 } }
    }
}

impl SudokuDigit {
    fn value(&self) -> Option<Digit> {
        self.is_assigned().then_some(self.value)
    }

    const fn is_assigned(&self) -> bool {
        self.value != 0
    }

    fn assign(&mut self, value: Digit) {
        self.value = value;
        self.candidates = 0;
    }

    fn single_candidate(&self) -> Option<Digit> {
        (self.candidates_len() == 1).then(|| Digit::try_from(u64::BITS - self.candidates.leading_zeros() - 1).expect("Unexpected number of bits"))
    }

    fn candidates(&self) -> Vec<Digit> {
        (1..=Digit::try_from(SUDOKU_SIZE).expect("Sudoku size should be reasonable")).filter(|&i| self.allows(i)).collect()
    }

    const fn candidates_len(&self) -> usize {
        self.candidates.count_ones() as usize
    }

    fn restrict(&mut self, value: Digit) {
        self.candidates &= !(1 << value);
    }

    const fn allows(&self, value: Digit) -> bool {
        self.candidates & 1 << value != 0
    }
}

// --- //

#[derive(Clone)]
struct Sudoku {
    sudoku: Vec<SudokuDigit>,
}

impl From<&str> for Sudoku {
    fn from(value: &str) -> Self {
        let mut sudoku = Self { sudoku: value.chars().map(char_as_i64).filter_map(|d| Digit::try_from(d).ok()).map(SudokuDigit::from).collect() };
        sudoku.init();
        sudoku
    }
}

impl Sudoku {
    fn values(&self) -> Vec<Digit> {
        self.sudoku.iter().filter_map(SudokuDigit::value).collect()
    }

    fn row(row: usize) -> impl Iterator<Item=usize> {
        (row * SUDOKU_SIZE..).take(SUDOKU_SIZE)
    }

    fn column(column: usize) -> impl Iterator<Item=usize> {
        (column..).step_by(SUDOKU_SIZE).take(SUDOKU_SIZE)
    }

    fn square(square: usize) -> impl Iterator<Item=usize> {
        let position = square % SUDOKU_SQUARE_SIZE * SUDOKU_SQUARE_SIZE + square / SUDOKU_SQUARE_SIZE * SUDOKU_SQUARE_SIZE * SUDOKU_SIZE;
        (0..SUDOKU_SQUARE_SIZE).flat_map(move |index| position + index * SUDOKU_SIZE..position + SUDOKU_SQUARE_SIZE + index * SUDOKU_SIZE)
    }

    fn restrict(&mut self, index: usize, value: Digit) {
        self.sudoku[index].restrict(value);
    }

    fn assign(&mut self, index: usize, value: Digit) {
        self.sudoku[index].assign(value);
        Self::row(index / SUDOKU_SIZE).filter(|&r| r != index).for_each(|r| self.sudoku[r].restrict(value));
        Self::column(index % SUDOKU_SIZE).filter(|&r| r != index).for_each(|r| self.sudoku[r].restrict(value));
        Self::square(index % SUDOKU_SIZE / SUDOKU_SQUARE_SIZE + SUDOKU_SQUARE_SIZE * (index / SUDOKU_SIZE / SUDOKU_SQUARE_SIZE)).filter(|&r| r != index).for_each(|r| self.sudoku[r].restrict(value));
    }

    fn assign_from_single(&mut self, index: usize) -> bool {
        self.sudoku[index].single_candidate().map_or(false, |single| {
            self.assign(index, single);
            true
        })
    }

    fn init(&mut self) {
        (0..self.sudoku.len()).for_each(|i| self.sudoku[i].value().iter().for_each(|&v| self.assign(i, v)));
    }

    fn is_solved(&self) -> bool {
        (0..self.sudoku.len()).all(|i| self.sudoku[i].is_assigned())
    }

    fn is_solvable(&self) -> bool {
        (0..self.sudoku.len()).all(|i| self.sudoku[i].is_assigned() || self.sudoku[i].candidates_len() > 0)
    }

    fn find_hidden_restrictions(&self, indexes: impl IntoIterator<Item=usize>) -> Vec<(usize, Digit)> {
        let positions = indexes.into_iter().collect::<Vec<_>>();
        (1..=Digit::try_from(SUDOKU_SIZE).expect("Sudoku size should be reasonable")).filter(|&value| positions.iter().filter(|&&i| self.sudoku[i].allows(value)).count() == 1).map(|single| positions.iter().find(|&&i| self.sudoku[i].allows(single)).map(|&i| (i, single)).expect("Hidden restriction should exist")).collect()
    }

    fn guess(&self) -> Option<(usize, Vec<Digit>)> {
        (0..self.sudoku.len()).filter(|&i| !self.sudoku[i].is_assigned()).min_by_key(|&i| self.sudoku[i].candidates_len()).map(|lowest| (lowest, self.sudoku[lowest].candidates()))
    }

    fn solve(&mut self) -> bool {
        loop {
            let mut elimination = (0..self.sudoku.len()).filter(|&i| self.assign_from_single(i)).count();
            if elimination != 0 { continue; }
            if self.is_solved() { break true; }

            // find hidden restrictions (restriction that only appear once in a set of digits)
            for i in 0..SUDOKU_SIZE {
                elimination += self.find_hidden_restrictions(Self::row(i)).iter().map(|&(index, value)| self.assign(index, value)).count();
                elimination += self.find_hidden_restrictions(Self::column(i)).iter().map(|&(index, value)| self.assign(index, value)).count();
                elimination += self.find_hidden_restrictions(Self::square(i)).iter().map(|&(index, value)| self.assign(index, value)).count();
            }
            if elimination != 0 { continue; }
            if self.is_solved() { break true; }

            // retry elimination as long as possible. when it doesn't succeed try guessing
            break self.is_solvable() && self.guess().map_or(false, |(guess, candidates)| {
                candidates.iter().any(|&candidate| {
                    let mut clone = self.clone();
                    clone.assign(guess, candidate);
                    if clone.solve() {
                        self.sudoku = clone.sudoku;
                        true
                    } else {
                        self.restrict(guess, candidate);
                        self.solve()
                    }
                })
            });
        }
    }
}
