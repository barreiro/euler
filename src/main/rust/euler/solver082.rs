// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::cmp::{min, Reverse};
use std::collections::BinaryHeap;
use std::fs::read_to_string;
use std::path::Path;

use euler::Solver;

// NOTE: This problem is a more challenging version of Problem 81.
//
// The minimal path sum in the 5 by 5 matrix below, by starting in any cell in the left column and finishing in any cell in the right column, and only moving up, down, and right, is indicated in red and bold; the sum is equal to 994.
//
// 131 673 234 103  18
// 201  96 342 965 150
// 630 803 746 422 111
// 537 699 497 121 956
// 805 732 524  37 331
//
// Find the minimal path sum from the left column to the right column in matrix.txt (right click and "Save Link/Target As..."), a 31K text file containing an 80 by 80 matrix.

const BASE_PATH: &str = "src/main/resources/net/projecteuler/barreiro/problem/";

pub struct Solver082 {
    pub n: isize,
    pub input: String,
}

impl Default for Solver082 {
    fn default() -> Self {
        let location = BASE_PATH.to_string() + "problem082-data.txt";
        let path = Path::new(location.trim());
        Solver082 { n: 80, input: read_to_string(path).expect("Unable to read file") }
    }
}

impl Solver for Solver082 {
    fn solve(&self) -> isize {
        let (mut matrix, last) = (str_to_matrix(&self), (self.n - 1) as usize);

        // fold the matrix right to left, and adjust when there is a 'shorter' path up or down
        (0..last).rev().for_each(|a| {
            let (original, prev) = ((0..=last).map(|i| matrix[i][a]).collect::<Vec<_>>(), a + 1);
            (0..=last).for_each(|i| {
                matrix[i][a] += min(matrix[i][prev], min(
                    if i == 0 { isize::max_value() } else { matrix[i - 1][a] },
                    if i == last { isize::max_value() } else { matrix[i + 1][a] + matrix[i + 1][prev] }));

                if matrix[i][a] != original[i] + matrix[i][prev] { // check if any of the already computed values can be 'relaxed'
                    let up = (0..i).rev().take_while(|&j| matrix[j][prev] > matrix[j + 1][a]).collect::<Vec<_>>();
                    up.iter().for_each(|&j| matrix[j][a] = min(matrix[j][a], original[j] + matrix[j + 1][a]));
                }
            });
        });
        (0..=last).map(|i| matrix[i][0]).min().unwrap()

        // Dijkstra algorithm ends up being slower than just folding the matrix right to left
        // _dijkstra(&str_to_matrix(&self), self.n as _)
    }
}

fn _dijkstra(matrix: &[Vec<isize>], dimension: usize) -> isize {
    let (capacity, last) = (dimension * dimension, dimension - 1);
    let expansion = |index| {
        let mut expansion = Vec::with_capacity(3);
        if index > dimension {
            expansion.push(index - dimension);
        }
        if index + dimension < capacity {
            expansion.push(index + dimension);
        }
        if index % dimension != last {
            expansion.push(index + 1);
        }
        expansion
    };

    let (mut priority_queue, mut g) = (BinaryHeap::with_capacity(capacity), vec![isize::max_value(); capacity]);
    (0..dimension).for_each(|i| {
        priority_queue.push(Reverse((matrix[i][0], dimension * i)));
        g[dimension * i] = matrix[i][0];
    });

    loop {
        let (_, current) = priority_queue.pop().unwrap().0;
        if current % dimension == last {
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

fn str_to_matrix(solver: &Solver082) -> Vec<Vec<isize>> {
    let mut parsed = vec![];
    for (l, line) in solver.input.split('\n').enumerate() {
        if l < solver.n as usize {
            parsed.push(line.split(',').take(solver.n as _).filter_map(|s| s.parse().ok()).collect());
        }
    }
    parsed
}
