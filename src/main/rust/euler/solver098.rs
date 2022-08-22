// COPYRIGHT (C) 2022 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

use algorithm::long::{from_digits, int_sqrt, pow_10, square, to_digits, unique_digits};
use algorithm::vec::cluster_by;
use euler::Solver;

// By replacing each of the letters in the word CARE with 1, 2, 9, and 6 respectively, we form a square number: 1296 = 36^2.
// What is remarkable is that, by using the same digital substitutions, the anagram, RACE, also forms a square number: 9216 = 96^2.
// We shall call CARE (and RACE) a square anagram word pair and specify further that leading zeroes are not permitted, neither may a different letter have the same digital value as another letter.
//
// Using words.txt (right click and 'Save Link/Target As...'), a 16K text file containing nearly two-thousand common English words, find all the square anagram word pairs (a palindromic word is NOT considered to be an anagram of itself).
//
// What is the largest square number formed by any member of such a pair?
//
// NOTE: All anagrams formed must be contained in the given text file.

const INPUT_FILE: &str = "src/main/resources/net/projecteuler/barreiro/problem/problem098-data.txt";

pub struct Solver098 {
    pub n: isize,
    pub input: String,
}

impl Default for Solver098 {
    fn default() -> Self {
        Solver098 { n: 1786, input: read_to_string(Path::new(INPUT_FILE)).expect("Unable to read file") }
    }
}

impl Solver for Solver098 {
    fn solve(&self) -> isize {
        let word_norm = |&word: &&str| {
            let mut sorted_chars = word.chars().collect::<Vec<_>>();
            sorted_chars.sort_unstable();
            sorted_chars
        };
        let _value_norm = |&value: &isize| {
            let mut sorted_digits = to_digits(value);
            sorted_digits.sort_unstable();
            sorted_digits
        };

        // closure to generate all the squares of a given len that have only unique digits
        // it's considered that all the words have distinct letters as well, which may not be always true
        let unique_squares_of_dim = |dim| (int_sqrt(pow_10((dim - 1) as _))..int_sqrt(pow_10(dim as _))).map(square).filter(|&sq| unique_digits(sq)).collect::<Vec<_>>();

        // closure to generate all the anagram squares of a given len that have only unique digits
        // it's considered that all the words have distinct letters as well, which may not be always true
        let _anagram_squares_of_dim = |dim: isize| {
            let mut permutations: HashMap<_, usize> = HashMap::new();
            let mut all_squares = (int_sqrt(pow_10(dim - 1))..int_sqrt(pow_10(dim))).map(square).filter(|&sq| unique_digits(sq)).collect::<Vec<_>>();
            all_squares.iter().for_each(|s| *permutations.entry(_value_norm(s)).or_default() += 1);
            all_squares.retain(|s| permutations[&_value_norm(s)] > 1);
            all_squares
        };

        let (words, mut squares_cache) = (self.input.split(',').map(|s| s.trim_matches('\"')).take(self.n as _).collect::<Vec<_>>(), HashMap::new());

        // find and sort words that are permutations of one another. iterate from longest to shortest
        let mut anagrams = cluster_by(&words, word_norm);
        anagrams.retain(|a| a.len() > 1);
        anagrams.sort_unstable_by_key(|pair| pair[0].len());
        anagrams.iter().rev().find_map(|pair| {
            let dim = pair[0].len();
            let mapping_indexes = pair[1].chars().map(|c| dim - 1 - pair[0].char_indices().find(|(_, d)| *d == c).unwrap().0).collect::<Vec<_>>();
            let remap = |value| {
                let digits = to_digits(value);
                from_digits(mapping_indexes.iter().map(|&i| digits[i]).rev().collect())
            };

            // finds if a number created from the mapped indexes (the mapping from the anagram) is also a square
            let squares = squares_cache.entry(dim).or_insert_with(|| unique_squares_of_dim(dim));
            squares.iter().rev().find_map(|&s| squares.binary_search(&remap(s)).is_ok().then(|| remap(s).max(s)))
        }).unwrap_or_default()
    }
}
