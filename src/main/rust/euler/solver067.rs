// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::collections::BinaryHeap;
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
const _ELEMENT_CEIL: isize = 100;

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
        let ceil = arithmetic_sum(self.n) as _;
        best_sum(0, 0, &str_to_heap(self.n, &self.input)[..ceil], &mut vec![0; ceil])
    }
}

fn _a_star_search(heap: Vec<isize>, target_level: isize) -> isize {
    let heap_index = |level, index| (arithmetic_sum(level) + index) as usize;

    // from[n] is the node immediately preceding it on the cheapest path from start to n currently known.
    let (mut priority_queue, mut g, mut from) = (BinaryHeap::with_capacity(heap.len()), vec![0; heap.len()], vec![usize::max_value(); heap.len()]);
    priority_queue.push((0, 0, 0));
    g[0] = heap[0];

    let leaf = loop {
        let (_, level, index) = priority_queue.pop().unwrap();
        let current = heap_index(level, index);
        if level == target_level {
            break current;
        }

        for &(child_level, child_index) in [(level + 1, index), (level + 1, index + 1)].iter() {
            // each iteration A* extends its paths based on the cost of the path and an (over-)estimate of the cost to the goal.
            let child = heap_index(child_level, child_index);
            let tentative = g[current] + heap[child];
            if tentative > g[child] {
                g[child] = tentative;
                from[child] = current;
                priority_queue.push((tentative + _ELEMENT_CEIL * (target_level - child_level), child_level, child_index));
            }
        }
    };

    // reconstruct path to the leaf based on the best known predecessor
    let (mut sum, mut position) = (heap[0], leaf);
    while from[position] != usize::max_value() {
        sum += heap[position];
        position = from[position];
    }
    sum
}

fn best_sum(level: isize, index: isize, heap: &[isize], cache: &mut [isize]) -> isize {
    let heap_index = (arithmetic_sum(level) + index) as usize;
    if heap_index < heap.len() && cache[heap_index] == 0 {
        cache[heap_index] = heap[heap_index] + best_sum(level + 1, index, heap, cache).max(best_sum(level + 1, index + 1, heap, cache))
    }
    *cache.get(heap_index).unwrap_or(&0)
}

// --- //

fn str_to_heap(level: isize, data: &String) -> Vec<isize> {
    let mut parsed = vec![];
    for (l, line) in data.split('\n').enumerate() {
        if l < level as usize {
            for s in line.split_whitespace() {
                if let Ok(value) = s.parse() {
                    parsed.push(value);
                }
            }
        }
    }
    parsed
}
