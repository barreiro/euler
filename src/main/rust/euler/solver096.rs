// COPYRIGHT (C) 2022 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::fs::read_to_string;
use std::path::Path;

use algorithm::cast::Cast;
use algorithm::long::{floor_sqrt_usize, from_digits};
use euler::Solver;

// Su Doku (Japanese meaning number place) is the name given to a popular puzzle concept.
// Its origin is unclear, but credit must be attributed to Leonhard Euler who invented a similar, and much more difficult, puzzle idea called Latin Squares.
// The objective of Su Doku puzzles, however, is to replace the blanks (or zeros) in a 9 by 9 grid in such that each row, column, and 3 by 3 box contains each of the digits 1 to 9.
//
// Below is an example of a typical starting puzzle grid and its solution grid.
//
//  0 0 3 | 0 2 0 | 6 0 0   4 8 3 | 9 2 1 | 6 5 7
//  9 0 0 | 3 0 5 | 0 0 1   9 6 7 | 3 4 5 | 8 2 1
//  0 0 1 | 8 0 6 | 4 0 0   2 5 1 | 8 7 6 | 4 9 3
//  ------+-------+------   ------+-------+------
//  0 0 8 | 1 0 2 | 9 0 0   5 4 8 | 1 3 2 | 9 7 6
//  7 0 0 | 0 0 0 | 0 0 8   7 2 9 | 5 6 4 | 1 3 8
//  0 0 6 | 7 0 8 | 2 0 0   1 3 6 | 7 9 8 | 2 4 5
//  ------+-------+------   ------+-------+------
//  0 0 2 | 6 0 9 | 5 0 0   3 7 2 | 6 8 9 | 5 1 4
//  8 0 0 | 2 0 3 | 0 0 9   8 1 4 | 2 5 3 | 7 6 9
//  0 0 5 | 0 1 0 | 3 0 0   6 9 5 | 4 1 7 | 3 8 2
// 
// A well constructed Su Doku puzzle has a unique solution and can be solved by logic, although it may be necessary to employ "guess and test" methods in order to eliminate options (there is much contested opinion over this).
// The complexity of the search determines the difficulty of the puzzle; the example above is considered easy because it can be solved by straight forward direct deduction.
//
// The 6K text file, sudoku.txt (right click and 'Save Link/Target As...'), contains fifty different Su Doku puzzles ranging in difficulty, but all with unique solutions (the first puzzle in the file is the example above).
//
// By solving all fifty puzzles find the sum of the 3-digit numbers found in the top left corner of each solution grid; for example, 483 is the 3-digit number found in the top left corner of the solution grid above.

const INPUT_FILE: &str = "src/main/resources/net/projecteuler/barreiro/problem/problem096-data.txt";
const SUDOKU_SIZE: usize = 9;
const SUDOKU_SQUARE_SIZE: usize = floor_sqrt_usize(SUDOKU_SIZE);

pub struct Solver096 {
    pub n: isize,
    pub input: String,
}

impl Default for Solver096 {
    fn default() -> Self {
        Solver096 { n: 50, input: read_to_string(Path::new(INPUT_FILE)).expect("Unable to read file") }
    }
}

impl Solver for Solver096 {
    fn solve(&self) -> isize {
        self.input.lines().map(|line| if line.starts_with("Grid") { "\n" } else { line }).collect::<String>().split_whitespace().take(self.n as _).map(|sudoku_str| {
            let mut sudoku = Sudoku::from(sudoku_str);
            sudoku.solve();

            from_digits(sudoku.values().iter().take(SUDOKU_SQUARE_SIZE).rev().copied().collect())
        }).sum()
    }
}

// --- //

// For SUDOKU_SIZE < 64 pack the allowed array in a usize
//
// #[derive(Clone)]
// struct SudokuExplicitDigit {
//     value: Option<isize>,
//     candidates: Vec<isize>,
// }
//
// impl From<isize> for SudokuExplicitDigit {
//     fn from(value: isize) -> Self {
//         if value == 0 { SudokuExplicitDigit { value: None, candidates: (1..=SUDOKU_SIZE as _).collect() } } else { SudokuExplicitDigit { value: Some(value), candidates: vec![] } }
//     }
// }
//
// impl SudokuExplicitDigit {
//     fn value(&self) -> Option<isize> {
//         self.value
//     }
//
//     fn is_assigned(&self) -> bool {
//         self.value.is_some()
//     }
//
//     fn assign(&mut self, value: isize) {
//         self.value = Some(value);
//         self.candidates.clear();
//     }
//
//     fn single_candidate(&mut self) -> Option<isize> {
//         self.candidates.first().copied().filter(|_| self.candidates.len() == 1)
//     }
//
//     fn candidates(&self) -> Vec<isize> {
//         self.candidates.to_vec()
//     }
//
//     fn candidates_len(&self) -> usize {
//         self.candidates.len()
//     }
//
//     fn restrict(&mut self, value: isize) {
//         self.candidates.retain(|&a| a != value)
//     }
//
//     fn allows(&self, value: isize) -> bool {
//         self.value.map_or(false, |v| v == value) || self.candidates.contains(&value)
//     }
// }

// --- //

#[derive(Clone)]
struct SudokuDigit {
    value: isize,
    candidates: usize,
}

impl From<isize> for SudokuDigit {
    fn from(value: isize) -> Self {
        Self { value, candidates: if value == 0 { (2 << SUDOKU_SIZE) - 2 } else { 0 } }
    }
}

impl SudokuDigit {
    fn value(&self) -> Option<isize> {
        self.is_assigned().then_some(self.value)
    }

    const fn is_assigned(&self) -> bool {
        self.value != 0
    }

    fn assign(&mut self, value: isize) {
        self.value = value;
        self.candidates = 0;
    }

    fn single_candidate(&mut self) -> Option<isize> {
        (self.candidates_len() == 1).then(|| (usize::BITS - self.candidates.leading_zeros() - 1) as _)
    }

    fn candidates(&self) -> Vec<isize> {
        (1..=SUDOKU_SIZE as _).filter(|&i| self.allows(i)).collect()
    }

    const fn candidates_len(&self) -> usize {
        self.candidates.count_ones() as usize
    }

    fn restrict(&mut self, value: isize) {
        self.candidates &= !(1 << value);
    }

    const fn allows(&self, value: isize) -> bool {
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
        let mut sudoku = Self { sudoku: value.chars().map(Cast::isize).map(SudokuDigit::from).collect() };
        sudoku.init();
        sudoku
    }
}

impl Sudoku {
    const fn _coordinates(index: usize) -> (usize, usize, usize) {
        (index / SUDOKU_SIZE, index % SUDOKU_SIZE, index % SUDOKU_SIZE / SUDOKU_SQUARE_SIZE + SUDOKU_SQUARE_SIZE * (index / SUDOKU_SIZE / SUDOKU_SQUARE_SIZE))
    }

    fn values(&self) -> Vec<isize> {
        self.sudoku.iter().map(|digit| digit.value().unwrap()).collect()
    }

    fn row(row: usize) -> impl Iterator<Item=usize> {
        (row * SUDOKU_SIZE..).take(SUDOKU_SIZE)
    }

    fn column(column: usize) -> impl Iterator<Item=usize> {
        (column..).step_by(SUDOKU_SIZE).take(SUDOKU_SIZE)
    }

    fn square(square: usize) -> impl Iterator<Item=usize> {
        let position = square % SUDOKU_SQUARE_SIZE * SUDOKU_SQUARE_SIZE + square / SUDOKU_SQUARE_SIZE * SUDOKU_SQUARE_SIZE * SUDOKU_SIZE;
        (0..SUDOKU_SQUARE_SIZE).flat_map(move |index| (position + index * SUDOKU_SIZE..position + SUDOKU_SQUARE_SIZE + index * SUDOKU_SIZE))
    }

    fn restrict(&mut self, index: usize, value: isize) {
        self.sudoku[index].restrict(value);
    }

    fn assign(&mut self, index: usize, value: isize) {
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
        for i in 0..self.sudoku.len() {
            (self.sudoku[i].is_assigned()).then(|| self.assign(i, self.sudoku[i].value().unwrap()));
        }
    }

    fn is_solved(&self) -> bool {
        (0..self.sudoku.len()).all(|i| self.sudoku[i].is_assigned())
    }

    fn is_solvable(&self) -> bool {
        (0..self.sudoku.len()).all(|i| self.sudoku[i].is_assigned() || self.sudoku[i].candidates_len() > 0)
    }

    fn find_hidden_restrictions(&mut self, indexes: impl Iterator<Item=usize>) -> Vec<(usize, isize)> {
        let positions = indexes.collect::<Vec<_>>();
        (1..=SUDOKU_SIZE as _).filter(|&value| positions.iter().filter(|&&i| self.sudoku[i].allows(value)).count() == 1).map(|single| positions.iter().find(|&&i| self.sudoku[i].allows(single)).map(|&i| (i, single)).unwrap()).collect()
    }

    fn guess(&mut self) -> Option<(usize, Vec<isize>)> {
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

    fn _print(&self) {
        println!("+-------+-------+-------+");
        (0..self.sudoku.len()).for_each(|i| {
            if i % SUDOKU_SIZE == 0 {
                print!("| ");
            }
            if self.sudoku[i].is_assigned() {
                print!("{:?} ", self.sudoku[i].value());
            } else {
                print!(". ");
            }
            if i % SUDOKU_SQUARE_SIZE == SUDOKU_SQUARE_SIZE - 1 {
                print!("| ");
            }
            if i % SUDOKU_SIZE == SUDOKU_SIZE - 1 {
                println!();
                if i / SUDOKU_SIZE % SUDOKU_SQUARE_SIZE == SUDOKU_SQUARE_SIZE - 1 {
                    println!("+-------+-------+-------+");
                }
            }
        });
        println!();
    }
}
