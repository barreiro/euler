// COPYRIGHT (C) 2022 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use crate::algorithm::cast::Cast;
use crate::algorithm::long::GetAndIncrement;
use crate::algorithm::vec::array_max;
use crate::Solver;

const VALUES: &[u64] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
const BULLS: &[u64] = &[25]; // these values only have double, no treble

/// In the game of darts a player throws three darts at a target board which is split into twenty equal sized sections numbered one to twenty.
///
/// [ image ]
///
/// The score of a dart is determined by the number of the region that the dart lands in. A dart landing outside the red/green outer ring scores zero.
/// The black and cream regions inside this ring represent single scores. However, the red/green outer ring and middle ring score double and treble scores respectively.
///
/// At the centre of the board are two concentric circles called the bull region, or bulls-eye. The outer bull is worth 25 points and the inner bull is a double, worth 50 points.
///
/// There are many variations of rules but in the most popular game the players will begin with a score 301 or 501 and the first player to reduce their running total to zero is a winner.
/// However, it is normal to play a "doubles out" system, which means that the player must land a double (including the double bulls-eye at the centre of the board) on their final dart to win; any other dart that would reduce their running total to one or lower means the score for that set of three darts is "bust".
///
/// When a player is able to finish on their current score it is called a "checkout" and the highest checkout is `170: T20 T20 D25` (two treble 20s and double bull).
///
/// There are exactly eleven distinct ways to check out on a score of 6:
/// ```
/// D3
/// D1 D2
/// S2 D2
/// D2 D1
/// S4 D1
/// S1 S1 D2
/// S1 T1 D1
/// S1 S3 D1
/// D1 D1 D1
/// D1 S2 D1
/// S2 S2 D1
/// ```
/// Note that `D1 D2` is considered different to `D2 D1` as they finish on different doubles. However, the combination `S1 T1 D1` is considered the same as `T1 S1 D1`.
///
/// In addition, we shall not include misses in considering combinations; for example, `D3` is the same as `0 D3` and `0 0 D3`.
///
/// Incredibly there are 42336 distinct ways of checking out in total.
///
/// How many distinct ways can a player checkout with a score less than 100?
pub struct Solver109 {
    pub n: u64,
}

impl Default for Solver109 {
    fn default() -> Self {
        Self { n: 100 }
    }
}

impl Solver for Solver109 {
    fn problem_name(&self) -> &str { "Darts" }

    fn solve(&self) -> i64 {
        let (doubles, trebles) = (VALUES.iter().chain(BULLS).map(|v| v * 2).collect::<Vec<_>>(), VALUES.iter().map(|v| v * 3).collect::<Vec<_>>());
        let all = VALUES.iter().chain(BULLS).chain(doubles.iter()).chain(trebles.iter()).copied().collect::<Vec<_>>();
        let mut checkouts = vec![0; 1 + array_max(&doubles).as_usize() + 2 * array_max(&trebles).as_usize()];
        
        for d in doubles {
            checkouts[d.as_usize()].get_and_increment(); // single dart checkout
            (0..all.len()).for_each(|i| {
                checkouts[(d + all[i]).as_usize()].get_and_increment(); // two dart checkout
                (i..all.len()).for_each(|j| { checkouts[(d + all[i] + all[j]).as_usize()].get_and_increment(); }); // three dart checkout
            });
        }
        checkouts.into_iter().take(self.n.as_usize()).sum()
    }
}
