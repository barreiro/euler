// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::cmp::{min, Reverse};
use std::collections::BinaryHeap;
use std::fs::read_to_string;
use std::path::Path;

use euler::Solver;

// In the 5 by 5 matrix below, the minimal path sum from the top left to the bottom right, by moving left, right, up, and down, is indicated in bold red and is equal to 2297.
//
// 131 673 234 103  18
// 201  96 342 965 150
// 630 803 746 422 111
// 537 699 497 121 956
// 805 732 524  37 331
//
// Find the minimal path sum from the top left to the bottom right by moving left, right, up, and down in matrix.txt (right click and "Save Link/Target As..."), a 31K text file containing an 80 by 80 matrix.

const BASE_PATH: &str = "src/main/resources/net/projecteuler/barreiro/problem/";

pub struct Solver083 {
    pub n: isize,
    pub input: String,
}

impl Default for Solver083 {
    fn default() -> Self {
        let location = BASE_PATH.to_string() + "problem083-data.txt";
        let path = Path::new(location.trim());
        Solver083 { n: 80, input: read_to_string(path).expect("Unable to read file") }
    }
}

impl Solver for Solver083 {
    fn solve(&self) -> isize {
        let (matrix, last) = (str_to_matrix(&self), (self.n - 1) as usize);
        let (mut sum, mut changes, mut modified) = (matrix.clone(), Vec::with_capacity(last * last), (0..=last).flat_map(|a| (0..=last).map(move |b| (a, b))).collect::<Vec<_>>());

        (1..=last).for_each(|i| { // preliminary step to fold the top and left borders
            sum[i][0] += sum[i - 1][0];
            sum[0][i] += sum[0][i - 1];
        });
        (1..=last).for_each(|a| (1..=last).for_each(|b| sum[a][b] += min(sum[a - 1][b], sum[a][b - 1])));

        // iterate over the sum matrix, finding better paths that go up or left.
        while modified.last() == Some(&(last, last)) {
            for (a, b) in modified {
                let candidate = min_neighbour(a, b, last, &sum) + matrix[a][b];
                if sum[a][b] > candidate {
                    sum[a][b] = candidate;
                    changes.push((a, b));
                }
            };
            modified = changes.drain(0..).collect();
        }
        sum[last][last]

        // Dijkstra algorithm ends up being slower than just folding the matrix from bottom right to top left
        // _dijkstra(&str_to_matrix(self), self.n as _)
    }
}

fn min_neighbour(a: usize, b: usize, last: usize, m: &[Vec<isize>]) -> isize {
    let (up, down) = (if a == 0 { isize::max_value() } else { m[a - 1][b] }, if a == last { isize::max_value() } else { m[a + 1][b] });
    let (left, right) = (if b == 0 { isize::max_value() } else { m[a][b - 1] }, if b == last { isize::max_value() } else { m[a][b + 1] });
    let (min_h, min_v) = (min(left, right), min(up, down));
    if up < min(down, min_h) {
        m[a - 1][b]
    } else if down < min(up, min_h) {
        m[a + 1][b]
    } else if right < min(left, min_v) {
        m[a][b + 1]
    } else {
        m[a][b - 1]
    }
}

fn _dijkstra(matrix: &[Vec<isize>], dimension: usize) -> isize {
    let (capacity, last) = (dimension * dimension, dimension - 1);
    let (mut priority_queue, mut g) = (BinaryHeap::with_capacity(capacity), vec![isize::max_value(); capacity]);
    priority_queue.push(Reverse((matrix[0][0], 0)));
    g[0] = matrix[0][0];

    let expansion = |index| {
        let mut expansion = Vec::with_capacity(4);
        if index > dimension {
            expansion.push(index - dimension);
        }
        if index + dimension < capacity {
            expansion.push(index + dimension);
        }
        if index % dimension != last {
            expansion.push(index + 1);
        }
        if index % dimension != 0 {
            expansion.push(index - 1);
        }
        expansion
    };

    loop {
        let (_, current) = priority_queue.pop().unwrap().0;
        if current == capacity - 1 {
            break g[current];
        }
        for &child in expansion(current).iter() {
            let tentative = g[current] + matrix[child / dimension][child % dimension];
            if tentative < g[child] {
                g[child] = tentative;
                priority_queue.push(Reverse((tentative, child)));
            }
        }
    }
}

// --- //

fn str_to_matrix(solver: &Solver083) -> Vec<Vec<isize>> {
    let mut parsed = vec![];
    for (l, line) in solver.input.split('\n').enumerate() {
        if l < solver.n as usize {
            parsed.push(line.split(',').take(solver.n as _).filter_map(|s| s.parse().ok()).collect());
        }
    }
    parsed
}
