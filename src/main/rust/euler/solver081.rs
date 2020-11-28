// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::cmp::{Reverse, min};
use std::collections::BinaryHeap;
use std::fs::read_to_string;
use std::path::Path;

use euler::Solver;

// In the 5 by 5 matrix below, the minimal path sum from the top left to the bottom right, by only moving to the right and down, is indicated in bold red and is equal to 2427.
//
// 131 673 234 103  18
// 201  96 342 965 150
// 630 803 746 422 111
// 537 699 497 121 956
// 805 732 524  37 331
//
// Find the minimal path sum from the top left to the bottom right by only moving right and down in matrix.txt (right click and "Save Link/Target As..."), a 31K text file containing an 80 by 80 matrix.

const BASE_PATH: &str = "src/main/resources/net/projecteuler/barreiro/problem/";

pub struct Solver081 {
    pub n: isize,
    pub input: String,
}

impl Default for Solver081 {
    fn default() -> Self {
        let location = BASE_PATH.to_string() + "problem081-data.txt";
        let path = Path::new(location.trim());
        Solver081 { n: 80, input: read_to_string(path).expect("Unable to read file") }
    }
}

impl Solver for Solver081 {
    fn solve(&self) -> isize {
        let (mut matrix, last) = (str_to_matrix(&self), (self.n - 1) as usize);
        (0..last).rev().for_each(|i| { // preliminary step to fold the right and bottom borders
            matrix[i][last] += matrix[i + 1][last];
            matrix[last][i] += matrix[last][i + 1];
        });
        (0..last).rev().for_each(|a| (0..last).rev().for_each(|b| matrix[a][b] += min(matrix[a + 1][b], matrix[a][b + 1])));
        matrix[0][0]

        // Dijkstra algorithm ends up being slower than just folding the matrix right to left
        // _dijkstra(&str_to_matrix(self), self.n as _)
    }
}

fn _dijkstra(matrix: &[Vec<isize>], dimension: usize) -> isize {
    let capacity = dimension * dimension;
    let expansion = |index| {
        let mut expansion = Vec::with_capacity(2);
        if index + dimension < capacity {
            expansion.push(index + dimension);
        }
        if index % dimension != dimension - 1 {
            expansion.push(index + 1);
        }
        expansion
    };

    let (mut priority_queue, mut g) = (BinaryHeap::with_capacity(capacity), vec![isize::max_value(); capacity]);
    priority_queue.push(Reverse((matrix[0][0], 0)));
    g[0] = matrix[0][0];

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

fn str_to_matrix(solver: &Solver081) -> Vec<Vec<isize>> {
    let mut parsed = vec![];
    for (l, line) in solver.input.split('\n').enumerate() {
        if l < solver.n as usize {
            parsed.push(line.split(',').take(solver.n as _).filter_map(|s| s.parse().ok()).collect());
        }
    }
    parsed
}
