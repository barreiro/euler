// COPYRIGHT (C) 2023 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::cast::Cast;
use Solver;

const INPUT_117: &[u64] = &[2, 3, 4];

/// Using a combination of grey square tiles and oblong tiles chosen from: red tiles (measuring two units), green tiles (measuring three units), and blue tiles (measuring four units), it is possible to tile a row measuring five units in length in exactly fifteen different ways.
/// ```
/// .....
/// RR...
/// .RR..
/// ..RR.
/// ...RR
/// rrRR.
/// rr.RR
/// .rrRR
/// GGG..
/// .GGG.
/// ..GGG
/// BBBB.
/// .BBBB
/// RRGGG
/// GGGRR
/// ```
/// How many ways can a row measuring fifty units in length be tiled?
///
/// NOTE: This is related to *Problem 116*.
pub struct Solver117 {
    pub n: u64,
    pub input: Vec<u64>,
}

impl Default for Solver117 {
    fn default() -> Self {
        Self { n: 50, input: INPUT_117.to_vec() }
    }
}

impl Solver for Solver117 {
    fn problem_name(&self) -> &str { "Red, green and blue Tiles" }

    fn solve(&self) -> i64 {
        multipart(self.n, &self.input).as_i64()
    }
}

// --- //

/// zero plus, for each block size, add the number of blocks that can be put before the current position
#[must_use]
pub fn multipart(total: u64, sizes: &[u64]) -> u64 {
    1 + multipart_memoize(total, sizes, &mut vec![None; 1 + total.as_usize()])
}

fn multipart_memoize(total: u64, sizes: &[u64], cache: &mut [Option<u64>]) -> u64 {
    if cache[total.as_usize()].is_none() {
        cache[total.as_usize()] = Some(sizes.iter().filter(|&&size| size <= total).map(|size| (0..=total - size).map(|position| 1 + multipart_memoize(position, sizes, cache)).sum::<u64>()).sum());
    }
    cache[total.as_usize()].expect("Cache should be populated")
}
