// COPYRIGHT (C) 2024 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::iter::from_fn;

use crate::algorithm::cast::Cast;
use crate::algorithm::filter::{equals_u64, less_than};
use crate::algorithm::long::{GetAndIncrement, Increment};
use crate::Solver;

/// The minimum number of cubes to cover every visible face on a cuboid measuring `3 x 2 x 1` is twenty-two.
///
/// [image]
///
/// If we then add a second layer to this solid it would require forty-six cubes to cover every visible face, the third layer would require seventy-eight cubes, and the fourth layer would require one-hundred and eighteen cubes to cover every visible face.
///
/// However, the first layer on a cuboid measuring `5 x 1 x 1` also requires twenty-two cubes; similarly the first layer on cuboids measuring `5 x 3 x 1`, `7 x 2 x 1`, and `11 x 1 x 1` all contain forty-six cubes.
///
/// We shall define `C(n)` to represent the number of cuboids that contain `n` cubes in one of its layers.
/// So `C(22) = 2`, `C(46) = 4`, `C(78) = 5` and `C(118) = 8`.
///
/// It turns out that `154` is the least value of `n` for which `C(n) = 10`.
///
/// Find the least value of `n` for which `C(n) = 1000`.
pub struct Solver126 {
    pub n: u64,
}

impl Default for Solver126 {
    fn default() -> Self {
        Self { n: 1_000 }
    }
}

impl Solver for Solver126 {
    fn problem_name(&self) -> &str { "Cuboid Layers" }

    fn solve(&self) -> i64 {
        // let ceil = 20 * self.n.as_i64(); // this experimental ceil seems to be adequate
        // let mut c_n = vec![0; ceil.as_usize()];
        //
        // (1..ceil / 4).for_each(|a| (1..=a).for_each(|b| (1..=b).for_each(|c| {
        //     cuboid_layers(a, b, c).take_while(less_than(ceil)).for_each(|cubes| c_n[cubes.as_usize()].increment())
        // })));
        // (0..c_n.len()).find(|&i| c_n[i] == self.n).as_i64()

        // let ceil = 20 * self.n.as_i64(); // this experimental ceil seems to be accurate
        // let mut c_n = vec![0; ceil.as_usize()];
        //
        // // function to get the number of cubes that cover a cuboid of dimensions `a x b x c` at a given `layer`
        // let cubes = |a, b, c, layer| 2 * (a * b + a * c + b * c) + 4 * (a + b + c + layer - 2) * (layer - 1);
        //
        // // functions to get maximum dimensions possible given a maximum number of cubes `n`
        // // these are driven from: `cubes(1, 1, 1, layer) ≤ n` solving for `layer`, `cubes(a, 1, 1, layer) ≤ n` solving for `a`, `cubes(a, b, 1, layer) ≤ n` solving for `b`, `cubes(a, b, c, layer) ≤ n` solving for `c`
        // let max_layers = |n| ceil_sqrt(n - 2) / 2;
        // let max_a = |layer, n| (n - 4 * layer * layer + 4 * layer - 2) / 4 * layer;
        // let max_b = |a, layer, n| (n - 4 * layer * a + 2 * a - 4 * layer * layer + 8 * layer - 4) / (2 * a + 4 * layer - 2);
        // let max_c = |a, b, layer, n| (n - 2 * a * b - 4 * layer * a + 4 * a - 4 * layer * b + 4 * b - 4 * layer * layer + 12 * layer - 8) / (2 * a + 2 * b + 4 * layer - 4);
        //
        // (1..max_layers(ceil)).for_each(|layer| (1..max_a(layer, ceil)).for_each(|a| (a..max_b(a, layer, ceil)).for_each(|b| (b..max_c(a, b, layer, ceil)).for_each(|c| c_n[cubes(a, b, c, layer).as_usize()].increment()))));
        // (0..c_n.len()).find(|&i| c_n[i] == self.n).as_i64()

        let ceil = 20 * self.n.as_i64(); // this experimental ceil seems to be adequate 
        let mut c_n = vec![0; ceil.as_usize()];

        // maximum dimensions possible are driven from: `cubes(1, 1, 1, 1) ≤ ceil` solving for `a`, `cubes(a, 1, 1, 1) ≤ ceil` solving for `b`, `cubes(a, b, 1, 1) ≤ ceil` solving for `c`
        (1..(ceil - 2) / 4).for_each(|a| (a..(ceil - 2 * a) / (2 * a + 2)).for_each(|b| (b..(ceil - 2 * a * b) / (2 * (a + b))).for_each(|c| {
            cuboid_layers(a, b, c).take_while(less_than(ceil)).for_each(|cubes| c_n[cubes.as_usize()].increment());
        })));
        c_n.iter().step_by(2).position(equals_u64(self.n)).as_i64() * 2 // taking advantage that `cubes()` is always even due to symmetry
    }
}

// --- //                

// iterates over the layers of a primitive cuboid and returns the number of cubes in each layer
fn _layers(a: u64, b: u64, c: u64) -> impl Iterator<Item=u64> {
    let mut layers = vec![(a, b, 0); c.as_usize()];
    from_fn(move || {
        let (first, last) = (*layers.first().expect("Cuboid should not be empty"), *layers.last().expect("Cuboid should not be empty"));
        for (a, b, l) in &mut layers {
            *a += 2;
            *b += 2;
            l.increment();
        }
        layers.insert(0, first);
        layers.push(last);

        // calculate the number of cubes
        Some(layers.iter().map(|&(a, b, l)| (l > 0).then(|| (a * b) - (a - 2) * (b - 2) - 4 * l).unwrap_or(a * b)).sum())
    })
}

// cubes (a, b, c, layer) = 2 * (a * b + a * c + b * c) + 4 * (a + b + c + layer - 2) * (layer - 1);
fn cuboid_layers(a: i64, b: i64, c: i64) -> impl Iterator<Item=i64> {
    let (sum_minus_two, cross_product_twice, mut layer) = (a + b + c - 2, 2 * (a * b + a * c + b * c), 1);
    from_fn(move || Some(cross_product_twice + 4 * (sum_minus_two + layer) * (layer.get_and_increment() - 1)))
}