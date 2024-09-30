// COPYRIGHT (C) 2023 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::cmp::Ordering::{Equal, Greater, Less};

use crate::algorithm::cast::Cast;
use crate::Solver;

const MIN_BLOCK_SIZE: u64 = 3;

/// A row measuring seven units in length has red blocks with a minimum length of three units placed on it, such that any two red blocks (which are allowed to be different lengths) are separated by at least one grey square.
/// There are exactly seventeen ways of doing this.
/// ```
/// .......
/// RRR....
/// .RRR...
/// ..RRR..
/// ...RRR.
/// ....RRR
/// RRRR...
/// .RRRR..
/// ..RRRR.
/// ...RRRR
/// RRRRR..
/// .RRRRR.
/// ..RRRRR
/// RRRRRR.
/// .RRRRRR
/// RRRRRRR
/// RRR.RRR
/// ```
/// How many ways can a row measuring fifty units in length be filled?
///
/// NOTE: Although the example above does not lend itself to the possibility, in general it is permitted to mix block sizes. For example, on a row measuring eight units in length you could use `red (3)`, `grey (1)`, and `red (4)`.
pub struct Solver114 {
    pub n: u64,
    pub min_block_size: u64,
}

impl Default for Solver114 {
    fn default() -> Self {
        Self { n: 50, min_block_size: MIN_BLOCK_SIZE }
    }
}

impl Solver for Solver114 {
    fn problem_name(&self) -> &str { "Counting block combinations I" }

    fn solve(&self) -> i64 {
        multipart(self.n, self.min_block_size).as_i64()
    }
}

// --- //

/// zero plus, for all the block sizes, add the number of blocks that can be put before the current position (with one of separation)
#[must_use]
pub fn multipart(total: u64, min_size: u64) -> u64 {
    1 + multipart_memoize(total, min_size, &mut vec![None; 1 + total.as_usize()])
}

fn multipart_memoize(total: u64, min_size: u64, cache: &mut [Option<u64>]) -> u64 {
    match total.cmp(&min_size) {
        Less => { 0 }
        Equal => { 1 }
        Greater => {
            if cache[total.as_usize()].is_none() {
                cache[total.as_usize()] = Some((min_size..=total).map(|size| (0..=total - size).map(|i| 1 + multipart_memoize(i.max(1) - 1, min_size, cache)).sum::<u64>()).sum());
            }
            cache[total.as_usize()].expect("Cache should be populated")
        }
    }
}
