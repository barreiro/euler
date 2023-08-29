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
/// Find the maximum total from top to bottom in `triangle.txt` (right click and 'Save Link/Target As...'), a 15K text file containing a triangle with one-hundred rows.
///
/// NOTE: This is a much more difficult version of *Problem 18*.
/// It is not possible to try every route to solve this problem, as there are `299` altogether!
/// If you could check one trillion (`10^12`) routes every second it would take over twenty billion years to check them all.
/// There is an efficient algorithm to solve it. ;o)
pub struct Solver067 {
    pub n: usize,
    pub input: String,
}

impl Default for Solver067 {
    fn default() -> Self {
        Self { n: 100, input: load_default_data(67) }
    }
}

impl Solver for Solver067 {
    fn problem_name(&self) -> &str { "Maximum path sum II" }

    fn solve(&self) -> i64 {
        let heap = self.input.lines().take(self.n).flat_map(|line| line.split_whitespace().filter_map(|s| s.parse::<i64>().ok())).collect::<Vec<_>>();
        best_sum(0, 0, &heap, &mut vec![0; heap.len()])
    }
}

fn best_sum(level: u64, index: u64, heap: &[i64], cache: &mut [i64]) -> i64 {
    let heap_index = (arithmetic_sum_u64(level) + index).as_usize();
    if heap_index < heap.len() && cache[heap_index] == 0 {
        cache[heap_index] = heap[heap_index] + best_sum(level + 1, index, heap, cache).max(best_sum(level + 1, index + 1, heap, cache));
    }
    *cache.get(heap_index).unwrap_or(&0)
}
