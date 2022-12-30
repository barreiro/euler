// COPYRIGHT (C) 2022 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::fs::read_to_string;
use std::path::Path;

const DEFAULT_DATA_BASE_PATH: &str = "src/main/resources/net/projecteuler/barreiro/problem/";
const DEFAULT_TEST_DATA_BASE_PATH: &str = "src/test/resources/net/projecteuler/barreiro/problem/";

/// loads the default data for a given problem
#[must_use]
pub fn load_default_data(problem: u64) -> String {
    read_to_string(Path::new(&format!("{}{}{:03?}{}", DEFAULT_DATA_BASE_PATH, "problem", problem, "-data.txt"))).expect("Unable to read file")
}

/// loads the default test data for a given problem
#[must_use]
pub fn load_default_test_data(problem: u64) -> String {
    read_to_string(Path::new(&format!("{}{}{:03?}{}", DEFAULT_TEST_DATA_BASE_PATH, "problem", problem, "-test-data.txt"))).expect("Unable to read file")
}

// --- //

/// convert an input string to a matrix
#[must_use]
pub fn str_to_matrix(input: &str, n: usize) -> Vec<Vec<i64>> {
    let mut parsed = vec![];
    for line in input.lines().take(n) {
        parsed.push(line.split(',').take(n).filter_map(|s| s.parse().ok()).collect());
    }
    parsed
}

