// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::continued_fraction::{add_mul, continued_expansion, convergent_with_expansion};
use euler::algorithm::long::{floor_sqrt, int_log_10, int_sqrt, is_even, pow_10, square};
use euler::algorithm::prime::generator_trial_division;
use euler::Solver;

// Consider quadratic Diophantine equations of the form:
//
// x ^ 2 – D * y ^ 2 = 1
//
// For example, when D = 13, the minimal solution in x is 649 ^ 2 – 13 * 180 ^ 2 = 1.
//
// It can be assumed that there are no solutions in positive integers when D is square.
//
// By finding minimal solutions in x for D = {2, 3, 5, 6, 7}, we obtain the following:
//
// 3 ^ 2 – 2 * 2 ^ 2 = 1
// 2 ^ 2 – 3 * 1 ^ 2 = 1
// 9 ^ 2 – 5 * 4 ^ 2 = 1
// 5 ^ 2 – 6 * 2 ^ 2 = 1
// 8 ^ 2 – 7 * 3 ^ 2 = 1
//
// Hence, by considering minimal solutions in x for D ≤ 7, the largest x is obtained when D = 5.
//
// Find the value of D ≤ 1000 in minimal solutions of x for which the largest value of x is obtained.

pub struct Solver066 {
    pub n: isize
}

impl Default for Solver066 {
    fn default() -> Self {
        Solver066 { n: 1000 }
    }
}

impl Solver for Solver066 {
    fn solve(&self) -> isize {
        let custom_comparator = |n: &Vec<_>| (n.len(), n[n.len() - 1]);

        // Algorithm is described in the paper "An Algorithm to Solve a Pell Equation" by Alexander Junod
        // http://www.geman.in/yahoo_site_admin/assets/docs/1_GMN-8492-V28N2.190180001.pdf
        let junod = |d| {
            let (mut previous, mut current, root, threshold) = ((vec![0], 1, d), (vec![1], 0, 1), floor_sqrt(d), pow_10(15));
            let sqrt = |n| if n == 0 { 0 } else { floor_sqrt(n) };
            while current.2 > 2 || current.1 == 0 {
                let (mut a_p, _, c_p) = previous;
                let (a, b, c) = current;
                let p_factor = sqrt(d - c * c_p);
                let q = (p_factor + root) / c;

                add_mul(&mut a_p, &a, q, threshold);
                previous = (a, b, c);
                current = (a_p, 1, c_p + 2 * q * p_factor - q * q * c); // ( q * a + a_p,  q * b + b_p, c_next)
            }
            if current.2 == 1 { // c is either 1 or 2
                add_mul(&mut current.0, &vec![0], 2, threshold);
            }
            current.0 // Outputs 2 * a / c instead of x ( actual value of x = ±1 + (2 * a^2) / c )
        };

        // the main idea is that any triple (a, b, k) (that is, one which satisfies a^2 - d * b^2 = k) can be composed
        // with the trivial triple (m, 1, m^2 - d) to get the new triple ((a * m) + (d * b), a + (b * m), k * (m^2 - d)) for any m
        let _chakravala = |d| {
            // the initial tuple holds (a, b, k), and starting with a solution with b = 1 we iterate until k = 1
            let (root, threshold, small_threshold) = (int_sqrt(d), pow_10(16), pow_10(9));
            let mut triple = (root, 1, square(root) - d);
            while triple.0 < threshold && triple.2 != 1 {
                let (a, b, k) = triple;

                // m is chosen so that (a + b * m) / k is an integer, and minimizes (m^2 - d) / k
                let m = (root - k.abs() / 2..=root + k.abs() / 2 + 1).find(|&m| (a + b * m) % k == 0).unwrap();
                triple = ((a * m + b * d) / k.abs(), (a + b * m) / k.abs(), (square(m) - d) / k);

                // Brahmagupta's observation can make an early return in cases k = +-1 +-2 +-4
                let (a, b, k) = triple;
                if (k == -1 || k.abs() == 2 || (k.abs() == 4 && (is_even(a) || is_even(b)))) && a < small_threshold {
                    triple = ((square(a) + d * square(b)) / k.abs(), 2 * a * b / k.abs(), 1);
                }
            }
            if triple.2 != 1 { Err("Overflow") } else { Ok((triple.0, triple.1)) }
        };

        // fallback method using continued fractions that can handle big numbers
        // the convergent of the period, if even, or twice if odd yields the the minimal solution
        let _convergent = |d| {
            let expansion = continued_expansion(d);
            let nth = if expansion.len() % 2 == 0 { 2 * expansion.len() - 2 } else { expansion.len() - 1 };
            convergent_with_expansion(expansion, nth as _)
        };

        // optimization: the length of x varies greatly (use it at expense of correctness)
        let _x_len = |d| { // TODO: change to map_or_else() as soon as the feature stabilizes
            let (x, _) = _chakravala(d).map(|(x, _)| (vec![x], vec![])).unwrap_or_else(|_| _convergent(d));
            x.iter().map(|&i| int_log_10(i)).sum::<isize>()
        };

//        return generator_trial_division().take_while(|&p| p < self.n as _).max_by_key(|&d| _x_len(d)).unwrap();

        // optimization: only check prime numbers because these generate the biggest numerators
        generator_trial_division().take_while(|&p| p < self.n as _).max_by_key(|&d| custom_comparator(&junod(d))).unwrap()
    }
}
