// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use crate::algorithm::cast::Cast;
use crate::algorithm::long::IncrementAndGet;
use crate::Solver;

/// It turns out that `12cm` is the smallest length of wire that can be bent to form an integer sided right angle triangle in exactly one way, but there are many more examples.
/// ```
/// 12cm: (3,4,5)
/// 24cm: (6,8,10)
/// 30cm: (5,12,13)
/// 36cm: (9,12,15)
/// 40cm: (8,15,17)
/// 48cm: (12,16,20)
/// ```
/// In contrast, some lengths of wire, like 20 cm, cannot be bent to form an integer sided right angle triangle, and other lengths allow more than one solution to be found; for example, using `120cm` it is possible to form exactly three different integer sided right angle triangles.
/// ```
/// 120cm: (30,40,50), (20,48,52), (24,45,51)
/// ```
/// Given that `L` is the length of the wire, for how many values of `L ≤ 1,500,000` can exactly one integer sided right angle triangle be formed?
pub struct Solver075 {
    pub n: usize,
}

impl Default for Solver075 {
    fn default() -> Self {
        Self { n: 1_500_000 }
    }
}

impl Solver for Solver075 {
    fn problem_name(&self) -> &str { "Singular integer right triangles" }

    fn solve(&self) -> i64 {
        // take advantage of the fact that the perimeter is always even
        let mut lengths = vec![0; self.n / 2 + 1];
        // the (2, 1) initial coprime pair ensures one value is even and the other is odd (as opposed to both odd)
        coprimes(2, 1, &mut lengths);
        lengths.into_iter().filter(|&l| l == 1).count().as_i64()
    }
}

// generates all perimeters using two coprime numbers (each coprime pair generates a primitive triplet)
fn coprimes(m: u64, n: u64, lengths: &mut [usize]) {
    let (sum, ceil) = ((m * (m + n)).as_usize(), lengths.len());
    if sum <= ceil {
        (sum..ceil).step_by(sum).for_each(|s| { lengths[s].increment_and_get(); });
        coprimes((m << 1) + n, m, lengths);
        coprimes((m << 1) - n, m, lengths);
        coprimes(m + (n << 1), n, lengths);
    }
}
