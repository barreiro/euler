// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::cast::Cast;
use algorithm::io::load_default_data;
use algorithm::long::arithmetic_sum_u64;
use Solver;

/// By starting at the top of the triangle below and moving to adjacent numbers on the row below, the maximum total from top to bottom is `23`.
/// ```
///    3   
///   7 4
///  2 4 6
/// 8 5 9 3
/// ```
/// That is, `3 + 7 + 4 + 9 = 23`.
/// Find the maximum total from top to bottom of the triangle below:
///
/// NOTE: As there are only `16384` routes, it is possible to solve this problem by trying every route.
/// However, *Problem 67*, is the same challenge with a triangle containing one-hundred rows; it cannot be solved by brute force, and requires a clever method! ;o)
pub struct Solver018 {
    pub n: usize,
    pub input: Vec<String>,
}

impl Default for Solver018 {
    fn default() -> Self {
        Self { n: 15, input: load_default_data(18).lines().map(String::from).collect() }
    }
}

impl Solver for Solver018 {
    fn solve(&self) -> i64 {
        let heap = str_to_heap(self.n, &self.input);
        best_sum(0, 0, &heap, &mut vec![0; heap.len()])
    }
}

fn best_sum(level: usize, index: usize, heap: &[i64], cache: &mut [i64]) -> i64 {
    let heap_index = arithmetic_sum_u64(level.as_u64()).as_usize() + index;
    if heap_index < heap.len() && cache[heap_index] == 0 {
        cache[heap_index] = heap[heap_index] + best_sum(level + 1, index, heap, cache).max(best_sum(level + 1, index + 1, heap, cache));
    }
    *cache.get(heap_index).unwrap_or(&0)
}

// --- //

fn str_to_heap(level: usize, data: &[String]) -> Vec<i64> {
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
