// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::Solver;

// It turns out that 12 cm is the smallest length of wire that can be bent to form an integer sided right angle triangle in exactly one way, but there are many more examples.
//
// 12 cm: (3,4,5)
// 24 cm: (6,8,10)
// 30 cm: (5,12,13)
// 36 cm: (9,12,15)
// 40 cm: (8,15,17)
// 48 cm: (12,16,20)
//
// In contrast, some lengths of wire, like 20 cm, cannot be bent to form an integer sided right angle triangle, and other lengths allow more than one solution to be found; for example, using 120 cm it is possible to form exactly three different integer sided right angle triangles.
//
// 120 cm: (30,40,50), (20,48,52), (24,45,51)
//
// Given that L is the length of the wire, for how many values of L ≤ 1,500,000 can exactly one integer sided right angle triangle be formed?

pub struct Solver075 {
    pub n: isize
}

impl Default for Solver075 {
    fn default() -> Self {
        Solver075 { n: 1_500_000 }
    }
}

impl Solver for Solver075 {
    fn solve(&self) -> isize {
        // solved with Euclides Formula --- a=m^2-n^2 --- b=2nm --- c=m^2+n^2 --- with m>n
//        let primitive_triplet = |m, n| if is_odd(n + m) && gcd(m, n) == 1 { Some((m * (m + n) << 1) as usize) } else { None };
//        let (ceil, mut lengths) = (floor_sqrt(self.n << 1) >> 1, vec![0; self.n as usize + 1]);
//        (2..=ceil).for_each(|m| (1..m).filter_map(|n| primitive_triplet(m, n)).for_each(|sum| (sum..).step_by(sum).take_while(|&s| s <= self.n as _).for_each(|s| lengths[s] += 1)));
//        lengths.iter().filter(|&&l| l == LENGTH).count() as _

//        let mut lengths = vec![0; self.n as usize + 1];
//        _primitive_triplets(3, 4, 5, &mut lengths);
//        lengths.iter().filter(|&&l| l == LENGTH).count() as _

        // take advantage of the fact that the perimeter is always even
        let mut lengths = vec![0; self.n as usize / 2 + 1];
        // the (2, 1) initial coprime pair ensures one value is even and the other is odd (as opposed to both odd)
        coprimes(2, 1, &mut lengths);
        lengths.iter().filter(|&&l| l == 1).count() as _
    }
}

// generates all primitive triplets using a Price matrix transformations
//fn _primitive_triplets(a: isize, b: isize, c: isize, lengths: &mut [usize]) {
//    let sum = (a + b + c) as usize;
//    if sum <= lengths.len() {
//        for i in 1..lengths.len() / sum {
//            lengths[i * sum] += 1;
//        }
//        _primitive_triplets((a << 1) + b - c, (-a + b + c) << 1, (-a << 1) + b + 3 * c, lengths);
//        _primitive_triplets((a << 1) + b + c, (a - b + c) << 1, (a << 1) - b + 3 * c, lengths);
//        _primitive_triplets((a << 1) - b + c, (a + b + c) << 1, (a << 1) + b + 3 * c, lengths);
//    }
//}

// generates all perimeters using two coprime numbers (each coprime pair generates a primitive triplet)
fn coprimes(m: isize, n: isize, lengths: &mut [usize]) {
    let (sum, ceil) = ((m * (m + n)) as usize, lengths.len());
    if sum <= ceil {
        (sum..ceil).step_by(sum).for_each(|s| lengths[s] += 1);
        coprimes((m << 1) + n, m, lengths);
        coprimes((m << 1) - n, m, lengths);
        coprimes(m + (n << 1), n, lengths);
    }
}

