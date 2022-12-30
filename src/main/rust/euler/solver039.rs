// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::cast::Cast;
use algorithm::combinatorics::pythagorean_triplets;
use algorithm::filter::less_or_equal_than_u64;
use algorithm::prime::primorals;
use algorithm::root::cube_u64;
use Solver;

/// If p is the perimeter of a right angle triangle with integral length sides, `{a,b,c}`, there are exactly three solutions for `p = 120`.
/// `{20,48,52}, {24,45,51}, {30,40,50}`
/// For which value of `p ≤ 1000`, is the number of solutions maximised?
pub struct Solver039 {
    pub n: u64,
}

impl Default for Solver039 {
    fn default() -> Self {
        Self { n: 1000 }
    }
}

#[allow(clippy::maybe_infinite_iter)]
impl Solver for Solver039 {
    fn solve(&self) -> i64 {
        // create a list of perimeters for some primitive triplets. use that to factorize the candidates
        // the upper bound `cube(a)` is aggressive, and not exhaustive as `n` increases
        let perimeters = pythagorean_triplets().take_while(|&(a, b, c)| c - b != 2 || cube_u64(a) <= self.n).map(|(a, b, c)| a + b + c).collect::<Vec<_>>();
        // perimeters.sort_unstable(); // should sort for correctness, but apparently we can get away without it (as triplets appear more or less sorted by perimeter)

        // primorals give the smallest numbers with the most factors. use multiples of them as candidates for maximum solutions (this proves to be a paramount optimization!)
        let steps = primorals().skip(1).map(|p| p * 2).take_while(less_or_equal_than_u64(self.n)).collect::<Vec<_>>();

        (0..).scan(0, |state, _| {
            *state += steps.iter().take_while(|&&s| s <= *state).last().unwrap_or(&steps[0]);
            Some(*state).filter(|&t| t <= self.n)
        }).max_by_key(|&candidate| perimeters.iter().take_while(|&&p| p <= candidate).filter(|&p| candidate % p == 0).count()).as_i64()
    }
}
