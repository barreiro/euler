// COPYRIGHT (C) 2023 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use crate::algorithm::cast::Cast;
use crate::algorithm::long::factorial;
use crate::Solver;

/// A bag contains one red disc and one blue disc. In a game of chance a player takes a disc at random and its colour is noted.
/// After each turn the disc is returned to the bag, an extra red disc is added, and another disc is taken at random.
///
/// The player pays `£1` to play and wins if they have taken more blue discs than red discs at the end of the game.
///
/// If the game is played for four turns, the probability of a player winning is exactly `11/120`, and so the maximum prize fund the banker should allocate for winning in this game would be `£10` before they would expect to incur a loss.
/// Note that any payout will be a whole number of pounds and also includes the original `£1` paid to play the game, so in the example given the player actually wins `£9`.
///
/// Find the maximum prize fund that should be allocated to a single game in which fifteen turns are played.
pub struct Solver121 {
    pub n: u64,
}

impl Default for Solver121 {
    fn default() -> Self {
        Self { n: 15 }
    }
}

impl Solver for Solver121 {
    fn problem_name(&self) -> &str { "Disc game prize fund" }

    fn solve(&self) -> i64 {
        (factorial(self.n + 1) / ways_to_win(0, 0, self.n)).as_i64()
    }
}

// --- //

// calculate the number of winning cases after taking a blue disk plus red disks times the number of winning cases after taking a red disk
fn ways_to_win(turn: u64, blues: u64, total: u64) -> u64 {
    if turn == total {
        u64::from(blues > total / 2)
    } else {
        ways_to_win(turn + 1, blues + 1, total) + (turn + 1) * ways_to_win(turn + 1, blues, total)
    }
}
