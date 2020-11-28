// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::fs::read_to_string;
use std::path::Path;

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
// Find the maximum total from top to bottom in triangle.txt (right click and 'Save Link/Target As...'), a 15K text file containing a triangle with one-hundred rows.
//
// NOTE: This is a much more difficult version of Problem 18.
// It is not possible to try every route to solve this problem, as there are 299 altogether!
// If you could check one trillion (1012) routes every second it would take over twenty billion years to check them all.
// There is an efficient algorithm to solve it. ;o)

const BASE_PATH: &str = "src/main/resources/net/projecteuler/barreiro/problem/";

pub struct Solver067 {
    pub n: isize,
    pub input: String,
}

impl Default for Solver067 {
    fn default() -> Self {
        let location = BASE_PATH.to_string() + "problem067-data.txt";
        let path = Path::new(location.trim());
        Solver067 { n: 100, input: read_to_string(path).expect("Unable to read file") }
    }
}

impl Solver for Solver067 {
    fn solve(&self) -> isize {
        let heap = str_to_heap(self.n, &self.input);
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

fn str_to_heap(level: isize, data: &str) -> Vec<isize> {
    let mut parsed = vec![];
    for (l, line) in data.split('\n').enumerate() {
        if l < level as usize {
            line.split_whitespace().filter_map(|s| s.parse().ok()).for_each(|value| parsed.push(value));
        }
    }
    parsed
}
