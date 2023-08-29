// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::cast::Cast;
use algorithm::digits::concatenation;
use Solver;

const OUTPUT_SQUARES: usize = 3;

const BOARD_DIM: usize = 40;
const CONSECUTIVE_DOUBLES: usize = 3;
const MARKOV_SIZE: usize = BOARD_DIM * CONSECUTIVE_DOUBLES;

// Jail squares
const JAIL: usize = 10;
const G2J: usize = 30;

// Community Chest
const CC: &[usize] = &[2, 17, 33];
const CC_CARDS: u32 = 16;
const CC_OUTCOME: &[fn(usize) -> usize] = &[|_| 0, |_| 10];

// Chance
const CH: &[usize] = &[7, 22, 36];
const CH_CARDS: usize = 16;
const CH_OUTCOME: &[fn(usize) -> usize] = &[|_| 0, |_| 10, |_| 11, |_| 24, |_| 39, |_| 5, |ch| CH_NEXT_R[CH.binary_search(&ch).expect("Should contain chance square")], |ch| CH_NEXT_R[CH.binary_search(&ch).expect("Should contain chance square")], |ch| CH_NEXT_U[CH.binary_search(&ch).expect("Should contain chance square")], |ch| ch - 3];
const CH_NEXT_R: &[usize] = &[15, 25, 5];
const CH_NEXT_U: &[usize] = &[12, 28, 12];

/// In the game, Monopoly, the standard board is set up in the following way:
/// ```
/// GO  A1  CC1 A2  T1  R1  B1  CH1 B2  B3  JAIL
/// H2                                        C1
/// T2                                        U1
/// H1                                        C2
/// CH3                                       C3
/// R4                                        R2
/// G3                                        D1
/// CC3                                      CC2
/// G2                                        D2
/// G1                                        D3
/// G2J F3  U2  F2  F1  R3  E3  E2  CH2  E1   FP
/// ```
/// A player starts on the `GO` square and adds the scores on two 6-sided dice to determine the number of squares they advance in a clockwise direction.
/// Without any further rules we would expect to visit each square with equal probability: `2.5%`.
/// However, landing on `G2J` (Go To Jail), `CC` (community chest), and `CH` (chance) changes this distribution.
///
/// In addition to `G2J`, and one card from each of `CC` and `CH`, that orders the player to go directly to jail, if a player rolls three consecutive doubles, they do not advance the result of their `3rd` roll. Instead they proceed directly to jail.
///
/// At the beginning of the game, the `CC` and `CH` cards are shuffled. When a player lands on `CC` or `CH` they take a card from the top of the respective pile and, after following the instructions, it is returned to the bottom of the pile.
/// There are sixteen cards in each pile, but for the purpose of this problem we are only concerned with cards that order a movement; any instruction not concerned with movement will be ignored and the player will remain on the `CC/CH` square.
///
/// Community Chest (`2/16` cards):
/// Advance to `GO`
/// Go to `JAIL`
///
/// Chance (`10/16` cards):
/// Advance to `GO`
/// Go to `JAIL`
/// Go to `C1`
/// Go to `E3`
/// Go to `H2`
/// Go to `R1`
/// Go to next `R` (railway company)
/// Go to next `R`
/// Go to next `U` (utility company)
/// Go back `3` squares.
///
/// The heart of this problem concerns the likelihood of visiting a particular square. That is, the probability of finishing at that square after a roll.
/// For this reason it should be clear that, with the exception of `G2J` for which the probability of finishing on it is zero, the `CH` squares will have the lowest probabilities, as `5/8` request a movement to another square, and it is the final square that the player finishes at on each roll that we are interested in.
/// We shall make no distinction between "Just Visiting" and being sent to `JAIL`, and we shall also ignore the rule about requiring a double to "get out of jail", assuming that they pay to get out on their next turn.
///
/// By starting at `GO` and numbering the squares sequentially from `00` to `39` we can concatenate these two-digit numbers to produce strings that correspond with sets of squares.
///
/// Statistically it can be shown that the three most popular squares, in order, are `JAIL` (`6.24%`) = Square `10`, `E3` (`3.18%`) = Square `24`, and `GO` (`3.09%`) = Square `00`.
/// So these three most popular squares can be listed with the six-digit modal string: `102400`.
///
/// If, instead of using two `6-sided` dice, two `4-sided` dice are used, find the six-digit modal string.
pub struct Solver084 {
    pub n: usize,
}

impl Default for Solver084 {
    fn default() -> Self {
        Self { n: 4 }
    }
}

impl Solver for Solver084 {
    fn problem_name(&self) -> &str { "Monopoly odds" }

    #[allow(clippy::cast_precision_loss)]
    fn solve(&self) -> i64 {
        let (mut transition, roll_probability) = (vec![vec![0.0; MARKOV_SIZE]; MARKOV_SIZE], 1.0 / (self.n * self.n) as f64);

        (0..MARKOV_SIZE).for_each(|square| {
            // fill probability of moving from square to square based on the dice roll
            let mut roll = vec![0.0; MARKOV_SIZE];
            (1..=self.n).for_each(|dice1| (1..=self.n).for_each(|dice2| {
                let landing = (square + dice1 + dice2) % BOARD_DIM + if dice1 == dice2 { (1 + square / BOARD_DIM) * BOARD_DIM } else { 0 };
                roll[if landing >= MARKOV_SIZE { JAIL } else { landing }] += roll_probability;
            }));

            // copy to the transition matrix, adjusting for special squares (Go To Jail, Community Chest, Chance)
            roll.into_iter().enumerate().filter(|&(_, p)| p != 0.0).for_each(|(target, p)| {
                let effective = target % BOARD_DIM;
                if effective == G2J {
                    transition[square][target - effective + JAIL] += p;
                } else if CC.contains(&effective) {
                    let p_fraction = p / f64::from(CC_CARDS);
                    transition[square][target] = p - p_fraction * CC_OUTCOME.len() as f64;
                    CC_OUTCOME.iter().for_each(|&outcome| transition[square][target - effective + outcome(effective)] += p_fraction);
                } else if CH.contains(&effective) {
                    let p_fraction = p / CH_CARDS as f64;
                    transition[square][target] = p - p_fraction * CH_OUTCOME.len() as f64;
                    CH_OUTCOME.iter().for_each(|&outcome| transition[square][target - effective + outcome(effective)] += p_fraction);
                } else {
                    transition[square][target] += p;
                }
            });
        });

        // the stationary distribution is given by the eigenvector of the transition matrix for the eigenvalue `1`. solve `( A - I ) * x = 0`
        let mut steady = (0..MARKOV_SIZE).map(|r| {
            let mut extended = (0..MARKOV_SIZE).map(|c| transition[c][r]).collect::<Vec<_>>();
            extended.push(0.0);
            extended[r] -= 1.0;
            extended
        }).collect::<Vec<_>>();

        // after transpose and append a `0.0` column, also add a line of 1.0 to ensure the sum of all probabilities equals `1.0`
        steady.push(vec![1.0; MARKOV_SIZE + 1]);

        // FORWARD ELIMINATION: reduce every element under diagonal to `0.0`
        (0..=MARKOV_SIZE).for_each(|c| {
            // PIVOTING: find the max value in column and swap rows. this stabilizes the algorithm
            let pivot = (c..=MARKOV_SIZE).max_by(|&r1, &r2| steady[r1][c].abs().partial_cmp(&steady[r2][c].abs()).expect("Entries in the Markov matrix should be finite")).expect("There should be a pivot row");
            steady.swap(c, pivot);

            (c + 1..=MARKOV_SIZE).for_each(|r| {
                let factor = steady[r][c] / steady[c][c];
                (c..=MARKOV_SIZE).for_each(|k| steady[r][k] -= factor * steady[c][k]);
            });
        });

        // REDUCTION: reduce elements over diagonal to `0.0`. pivoting ensures last row is all `0.0`
        (1..MARKOV_SIZE).rev().for_each(|c| {
            // optimization: only compute last column
            (0..c).rev().for_each(|r| steady[r][MARKOV_SIZE] -= steady[r][c] / steady[c][c] * steady[c][MARKOV_SIZE]);
            steady[c][MARKOV_SIZE] /= steady[c][c];
        });

        // take the last column of the steady state, group consecutive rolls, sort and output
        let mut ordered = (0..BOARD_DIM).map(|target| (0..CONSECUTIVE_DOUBLES).map(|d| steady[target + d * BOARD_DIM][MARKOV_SIZE]).sum::<f64>()).enumerate().collect::<Vec<_>>();
        ordered.sort_unstable_by(|&(_, i), &(_, j)| j.total_cmp(&i));
        ordered.iter().take(OUTPUT_SQUARES).fold(0, |acc, &(x, _)| if x == 0 { acc * 100 } else if x < 10 { concatenation(acc * 10, x.as_u64()) } else { concatenation(acc, x.as_u64()) }).as_i64()
    }
}
