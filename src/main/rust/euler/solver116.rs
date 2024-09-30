// COPYRIGHT (C) 2023 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::cmp::Ordering::{Equal, Greater, Less};

use crate::algorithm::cast::{Cast, to_i64};
use crate::Solver;

const INPUT_116: &[u64] = &[2, 3, 4];

/// A row of five grey square tiles is to have a number of its tiles replaced with coloured oblong tiles chosen from red (length two), green (length three), or blue (length four).
///
/// If red tiles are chosen there are exactly seven ways this can be done.
/// ```
/// RR...
/// .RR..
/// ..RR.
/// ...RR
/// rrRR.
/// rr.RR
/// .rrRR
/// ```
/// If green tiles are chosen there are three ways.
/// ```
/// GGG..
/// .GGG.
/// ..GGG
/// ```
/// And if blue tiles are chosen there are two ways.
/// ```
/// BBBB.
/// .BBBB
/// ```
/// Assuming that colours cannot be mixed there are `7 + 3 + 2 = 12` ways of replacing the grey tiles in a row measuring five units in length.
///
/// How many different ways can the grey tiles in a row measuring fifty units in length be replaced if colours cannot be mixed and at least one coloured tile must be used?
///
/// NOTE: This is related to *Problem 117*.
pub struct Solver116 {
    pub n: u64,
    pub input: Vec<u64>,
}

impl Default for Solver116 {
    fn default() -> Self {
        Self { n: 50, input: INPUT_116.to_vec() }
    }
}

impl Solver for Solver116 {
    fn problem_name(&self) -> &str { "Red, green or blue Tiles" }

    #[allow(clippy::maybe_infinite_iter)]
    fn solve(&self) -> i64 {
        self.input.iter().map(|&size| multipart(self.n, size)).map(to_i64).sum()
    }
}

// --- //

/// add the number of blocks that can be put before the current position
#[must_use]
pub fn multipart(total: u64, size: u64) -> u64 {
    multipart_memoize(total, size, &mut vec![None; 1 + total.as_usize()])
}

fn multipart_memoize(total: u64, size: u64, cache: &mut [Option<u64>]) -> u64 {
    match total.cmp(&size) {
        Less => { 0 }
        Equal => { 1 }
        Greater => {
            if cache[total.as_usize()].is_none() {
                cache[total.as_usize()] = Some((0..=total - size).map(|position| 1 + multipart_memoize(position, size, cache)).sum());
            }
            cache[total.as_usize()].expect("Cache should be populated")
        }
    }
}
