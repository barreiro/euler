// COPYRIGHT (C) 2023 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::cast::Cast;
use Solver;

/// The most naive way of computing `n^15` requires fourteen multiplications:
/// ```
/// n * n * ... * n = n^15
/// ```
/// But using a "binary" method you can compute it in six multiplications:
/// ```
/// n * n = n^2
/// n^2 * n^2 = n^4
/// n^4 * n^4 = n^4
/// n^8 * n^4 = n^12
/// n^12 * n^2 = n^14
/// n^14 * n = n^15
/// ```
/// However it is yet possible to compute it in only five multiplications:
/// ```
/// n * n = n^2
/// n^2 * n = n^3
/// n^3 * n^3 = n^6
/// n^6 * n^6 = n^12
/// n^12 * n^3 = n^15
/// ```
/// We shall define `m(k)` to be the minimum number of multiplications to compute `n^k`; for example `m(15) = 5`.
///
/// For `1 ≤ k ≤ 200`, find `∑ m(k)`.
pub struct Solver122 {
    pub n: u64,
}

impl Default for Solver122 {
    fn default() -> Self {
        Self { n: 200 }
    }
}

impl Solver for Solver122 {
    fn problem_name(&self) -> &str { "Efficient exponentiation" }

    fn solve(&self) -> i64 {
        // let mut cache = &mut vec![None; 1 + self.n.as_usize()];
        // (2..=self.n).map(|n| _multiplications(n, &mut cache)[0].len().as_i64()).sum()

        let minimum_lengths = &mut vec![None; 1 + self.n.as_usize()];
        addition_chains(2, &mut vec![1], minimum_lengths);
        minimum_lengths.iter().map(|len| len.unwrap_or_default().as_i64()).sum()
    }
}

// --- //

// find a partition of value with the minimum number of elements that includes 1 and sum to value
fn _multiplications(value: u64, cache: &mut [Option<Vec<Vec<u64>>>]) -> Vec<Vec<u64>> {
    if value == 1 {
        vec![vec![]]
    } else {
        if cache[value.as_usize()].is_none() {
            // attempt to restrict the search space
            let mut min_len = cache[(if value % 2 == 0 { value / 2 } else { value - 1 }).as_usize()].as_ref().map_or(usize::MAX, |s| s[0].len() + 1);
            let mut min = (1..=value / 2).flat_map(|p| {
                let size_filter = |s: &Vec<_>| min_len == usize::MAX || s.len() < min_len;
                let mut set = _multiplications(value - p, cache).into_iter().filter(size_filter).flat_map(|other| {
                    _multiplications(p, cache).into_iter().filter(size_filter).map(|mut s| {
                        s.extend_from_slice(&other);
                        s
                    }).collect::<Vec<_>>()
                }).collect::<Vec<_>>();

                for s in &mut set {
                    s.sort_unstable();
                    s.dedup();
                    s.push(value);
                }
                min_len = min_len.min(set.iter().map(Vec::len).min().unwrap_or(usize::MAX));
                set.retain(|s| s.len() == min_len);
                set
            }).collect::<Vec<_>>();
            min.retain(|s| s.len() == min_len);
            cache[value.as_usize()] = Some(min);
        }
        cache[value.as_usize()].clone().expect("Cache should be populated")
    }
}

// --- //

// following section 3 of the paper "Efficient generation of minimal length addition chains" by Edward G. Thurber
// see http://wwwhomes.uni-bielefeld.de/achim/siam_thurber.pdf
// it's still not very efficient, but is reasonable in the context of the problem
fn addition_chains(value: u64, stack: &mut Vec<u64>, lengths: &mut [Option<usize>]) {
    // expand when `minimal == stack.len()` is necessary in order to visit all the necessary nodes in the search space
    if lengths[value.as_usize()].filter(|&minimal| minimal < stack.len()).is_none() {
        lengths[value.as_usize()] = Some(stack.len());

        stack.push(value);
        // important to calculate large values first to speed up the population of `lengths`
        (0..stack.partition_point(|&s| (value + s).as_usize() < lengths.len())).rev().for_each(|i| addition_chains(value + stack[i], stack, lengths));
        stack.pop();
    }
}
