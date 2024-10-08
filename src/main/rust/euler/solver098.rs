// COPYRIGHT (C) 2022 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::collections::HashMap;

use crate::algorithm::cast::Cast;
use crate::algorithm::digits::{Digits, from_raw_digits};
use crate::algorithm::filter::is_unique_digits;
use crate::algorithm::io::load_default_data;
use crate::algorithm::root::{floor_sqrt_u64, pow_10, square_u64};
use crate::algorithm::vec::cluster_by;
use crate::Solver;

/// By replacing each of the letters in the word `CARE` with `1, 2, 9, and 6` respectively, we form a square number: `1296 = 36^2`.
/// What is remarkable is that, by using the same digital substitutions, the anagram, `RACE`, also forms a square number: `9216 = 96^2`.
/// We shall call `CARE` (and `RACE`) a square anagram word pair and specify further that leading zeroes are not permitted, neither may a different letter have the same digital value as another letter.
///
/// Using `words.txt` (right click and 'Save Link/Target As...'), a 16K text file containing nearly two-thousand common English words, find all the square anagram word pairs (a palindromic word is NOT considered to be an anagram of itself).
///
/// What is the largest square number formed by any member of such a pair?
///
/// NOTE: All anagrams formed must be contained in the given text file.
pub struct Solver098 {
    pub n: usize,
    pub input: String,
}

impl Default for Solver098 {
    fn default() -> Self {
        Self { n: 1786, input: load_default_data(98) }
    }
}

impl Solver for Solver098 {
    fn problem_name(&self) -> &str { "Anagramic squares" }

    fn solve(&self) -> i64 {
        let word_norm = |&word: &&str| {
            let mut sorted_chars = word.chars().collect::<Vec<_>>();
            sorted_chars.sort_unstable();
            sorted_chars
        };

        // closure to generate all the squares of a given len that have only unique digits
        // it's considered that all the words have distinct letters as well, which may not be always true
        let unique_squares_of_dim = |dim| (floor_sqrt_u64(pow_10((dim - 1) as u64))..floor_sqrt_u64(pow_10(dim as u64))).map(square_u64).filter(is_unique_digits).collect::<Vec<_>>();

        let (words, mut squares_cache) = (self.input.split(',').map(|s| s.trim_matches('\"')).take(self.n).collect::<Vec<_>>(), HashMap::new());

        // find and sort words that are permutations of one another. iterate from longest to shortest
        let mut anagrams = cluster_by(&words, word_norm);
        anagrams.retain(|a| a.len() > 1);
        anagrams.sort_unstable_by_key(|pair| pair[0].len());
        anagrams.into_iter().rev().find_map(|pair| {
            let dim = pair[0].len();
            let mapping_indexes = pair[1].chars().map(|c| dim - 1 - pair[0].char_indices().find(|&(_, d)| d == c).expect("There should be a matching word").0).collect::<Vec<_>>();
            let remap = |value| {
                let digits = Digits::from(value);
                from_raw_digits(&mapping_indexes.iter().filter_map(|&i| digits.get(i).copied()).rev().collect::<Vec<_>>())
            };

            // finds if a number created from the mapped indexes (the mapping from the anagram) is also a square
            let squares = squares_cache.entry(dim).or_insert_with(|| unique_squares_of_dim(dim));
            squares.iter().rev().find_map(|&s| squares.binary_search(&remap(s)).is_ok().then(|| remap(s).max(s)))
        }).as_i64()
    }
}
