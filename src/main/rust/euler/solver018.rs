// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::long::arithmetic_sum;
use euler::Solver;

// By starting at the top of the triangle below and moving to adjacent numbers on the row below, the maximum total from top to bottom is 23.
//
//    3
//   7 4
//  2 4 6
// 8 5 9 3
//
// That is, 3 + 7 + 4 + 9 = 23.
// Find the maximum total from top to bottom of the triangle below:
//
// NOTE: As there are only 16384 routes, it is possible to solve this problem by trying every route.
// However, Problem 67, is the same challenge with a triangle containing one-hundred rows; it cannot be solved by brute force, and requires a clever method! ;o)

const INPUT_018: &[&str; 15] = &[
    "75",
    "95 64",
    "17 47 82",
    "18 35 87 10",
    "20 04 82 47 65",
    "19 01 23 75 03 34",
    "88 02 77 73 07 63 67",
    "99 65 04 28 06 16 70 92",
    "41 41 26 56 83 40 80 70 33",
    "41 48 72 33 47 32 37 16 94 29",
    "53 71 44 65 25 43 91 52 97 51 14",
    "70 11 33 28 77 73 17 78 39 68 17 57",
    "91 71 52 38 17 14 91 43 58 50 27 29 48",
    "63 66 04 68 89 53 67 30 73 16 69 87 40 31",
    "04 62 98 27 23 09 70 98 73 93 38 53 60 04 23"];

pub struct Solver018<'a> {
    pub n: isize,
    pub input: Vec<&'a str>,
}

impl<'a> Default for Solver018<'a> {
    fn default() -> Self {
        Solver018 { n: 15, input: INPUT_018.to_vec() }
    }
}

impl<'a> Solver for Solver018<'a> {
    fn solve(&self) -> isize {
        let heap = str_to_heap(self.n as _, &self.input);
        best_sum(0, 0, &heap, &mut vec![0; heap.len()])
    }
}

fn best_sum(level: isize, index: isize, heap: &[isize], cache: &mut [isize]) -> isize {
    let heap_index = (arithmetic_sum(level) + index) as usize;
    if heap_index < heap.len() && cache[heap_index] == 0 {
        cache[heap_index] = heap[heap_index] + best_sum(level + 1, index, heap, cache).max(best_sum(level + 1, index + 1, heap, cache))
    }
    *cache.get(heap_index).unwrap_or(&0)
}

// --- //

fn str_to_heap(level: usize, data: &[&str]) -> Vec<isize> {
    let mut parsed = vec![];
    for (l, line) in data.iter().enumerate() {
        if l < level {
            for s in line.split_whitespace() {
                if let Ok(value) = s.parse() {
                    parsed.push(value);
                }
            }
        }
    }
    parsed
}
