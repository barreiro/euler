// COPYRIGHT (C) 2024 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use crate::algorithm::cast::Cast;
use crate::algorithm::long::arithmetic_sum_u64;
use crate::algorithm::prime::PrimeTestWithCache;
use crate::Solver;

const _TARGET_VALUE: usize = 3;

// values from https://www.redblobgames.com/grids/hexagons/#neighbors-axial
const _NEIGHBOURS: &[(i64, i64)] = &[(1, -1), (0, -1), (-1, 0), (-1, 1), (0, 1), (1, 0)]; // down-left, bottom, down-right, up-right, top, up-left
const _PRIME_DIFFERENCE_NEIGHBOURS_ONE: &[usize] = &[0, 2, 5]; // down-left, down-right, up-left
const _PRIME_DIFFERENCE_NEIGHBOURS_FIRST: &[usize] = &[2, 3, 5]; // down-right, up-right, up-left
const _PRIME_DIFFERENCE_NEIGHBOURS_LAST: &[usize] = &[0, 3, 5]; // down-left, up-right, up-left

/// A hexagonal tile with number `1` is surrounded by a ring of six hexagonal tiles, starting at "12 o'clock" and numbering the tiles `2` to `7` in an anti-clockwise direction.
///
/// New rings are added in the same fashion, with the next rings being numbered `8` to `19`, `20` to `37`, `38` to `61`, and so on. The diagram below shows the first three rings.
/// ```
///             20
///         21      37
///     22       8      36
/// 23       9      19      35
///     10       2      18
/// 24       3       7      34
///     11       1      17
/// 25       4       6      33
///     12       5      16
/// 26      13      15      32
///     27      14      31
///         28      30
///             29
/// ```
/// By finding the difference between tile `n` and each of its six neighbours we shall define `PD(n)` to be the number of those differences which are prime.
///
/// For example, working clockwise around tile `8` the differences are `12`, `29`, `11`, `6`, `1` and `13`. So `PD(8) = 3`.
///
/// In the same way, the differences around tile `17` are `1`, `17`, `16`, `1`, `11` and `10`, hence `PD(17) = 2`.
///
/// It can be shown that the maximum value of `PD(n)` is `3`.
///
/// If all the tiles for which `PD(n) = 3` are listed in ascending order to form a sequence, the `10th` tile would be `217`.
///
/// Find the `2000th` tile in this sequence.
pub struct Solver128 {
    pub n: usize,
}

impl Default for Solver128 {
    fn default() -> Self {
        Self { n: 2_000 }
    }
}

impl Solver for Solver128 {
    fn problem_name(&self) -> &str { "Hexagonal Tile Differences" }

    fn solve(&self) -> i64 {
        // let tester = PrimeTestWithCache::default();

        // let spiral_to_axial = |(layer, position): (i64, i64)| { // convert between spiral and axial coordinate systems. axial is better to generate neighbours
        //     let (direction, scale) = if layer == 0 { (0, 0) } else { ((position / layer).as_usize(), (position % layer).as_i64()) };
        //     let (along, out) = (NEIGHBOURS[direction], NEIGHBOURS[(direction + 4) % 6]);
        //
        //     (along.0 * scale + out.0 * layer.as_i64(), along.1 * scale + out.1 * layer.as_i64())
        // };

        // let neighbours = |ax: (i64, i64)| { // only three of the neighbours can generate prime differences, the other three are either one or even by design
        //     (if ax.1 == 0 { // 1 is a special case, as it's both first and last
        //         PRIME_DIFFERENCE_NEIGHBOURS_ONE
        //     } else if ax.0 == 0 { // 2, 8, 20…
        //         PRIME_DIFFERENCE_NEIGHBOURS_FIRST
        //     } else { // 7, 19, 37…
        //         PRIME_DIFFERENCE_NEIGHBOURS_LAST
        //     }).iter().map(|&n| (ax.0 + NEIGHBOURS[n].0, ax.1 + NEIGHBOURS[n].1)).collect::<Vec<_>>()
        // };

        // let decode = |&ax: &(i64, i64)| {
        //     let layer = ax.0.abs().max(ax.1.abs()).max((-ax.0 - ax.1).abs());
        //     (if ax.0 >= 0 { // calculate the position based on the changing coordinate for each sextant
        //         if ax.1 >= 0 {
        //             ax.0
        //         } else if ax.0 >= -ax.1 {
        //             layer - ax.1
        //         } else {
        //             3 * layer - ax.0
        //         }
        //     } else if ax.1 <= 0 {
        //         3 * layer - ax.0
        //     } else if -ax.0 >= ax.1 {
        //         4 * layer + ax.1
        //     } else {
        //         6 * layer + ax.0
        //     }) + (0..layer).scan(1, |state, l| { // calculate the value of position 0 of a given layer
        //         *state += 6 * l;
        //         Some(*state + 1)
        //     }).last().unwrap_or(1)
        // };

        // consider only the first and last tile of every ring
        // first generate those in spiral coordinates, then convert to axial coordinates https://www.redblobgames.com/grids/hexagons/#coordinates-axial
        // (0..).flat_map(|layer| [(layer, if layer == 0 { 0 } else { 6 * layer - 1 }), (layer + 1, 0)])
        //     .map(spiral_to_axial)
        //     .filter_map(|ax| Some(decode(&ax)).filter(|&d| neighbours(ax).iter().map(decode).map(|n| d.abs_diff(n))
        //         .filter(|&diff| tester.is_prime(diff)).count() == TARGET_VALUE))
        //     .nth(self.n - 1).as_i64();

        let tester = PrimeTestWithCache::default();
        
        // calculate the differences, depending on if it's the first or last position in a layer
        // in each case, two of those differences are multiples of six (top and bottom) and another is one 
        // 
        //            6*l                           6*l + 6  
        // 6*l + 1          12*l + 5        6*l - 1         6*l + 5
        //             0               or              0
        //    1              6*l - 1       12*l - 7            1
        //          6*l - 6                           6*l
        //
        let three_prime_differences = |&(layer, position): &(u64, u64)| {
            if position == 0 {
                [6 * layer - 1, 6 * layer + 1, 12 * layer + 5]
            } else {
                [6 * layer - 1, 6 * layer + 5, 12 * layer - 7]
            }.into_iter().all(|d| tester.is_prime(d))
        };
        
        let spiral_to_value = |(layer, position)| 2 + position + 6 * arithmetic_sum_u64(layer - 1);

        // consider only the first and last position on each layer
        (1..).flat_map(|layer| [(layer, 0), (layer, 6 * layer - 1)]).filter(three_prime_differences).nth(self.n - 1).map(spiral_to_value).as_i64()
    }
}
