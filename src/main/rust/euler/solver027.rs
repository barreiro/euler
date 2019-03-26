// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::long::square;
use euler::algorithm::prime::miller_rabin;
use euler::algorithm::prime::PrimeGenerator;
use euler::algorithm::prime::PrimesLessThan;
use euler::Solver;

// Euler discovered the remarkable quadratic formula: n^2 + n + 41
// It turns out that the formula will produce 40 primes for the consecutive values n = 0 to 39.
// However, when n = 40, 402 + 40 + 41 = 40(40 + 1) + 41 is divisible by 41, and certainly when n = 41, 41^2 + 41 + 41 is clearly divisible by 41.
// The incredible formula  n^2 − 79n + 1601 was discovered, which produces 80 primes for the consecutive values n = 0 to 79.
// The product of the coefficients, −79 and 1601, is −126479.
//
// Considering quadratics of the form: n^2 + an + b, where |a| < 1000 and |b| < 1000 where |n| is the modulus/absolute value of n e.g. |11| = 11 and |−4| = 4
// Find the product of the coefficients, a and b, for the quadratic expression that produces the maximum number of primes for consecutive values of n, starting with n = 0.

const HEEGNER: isize = -163;

pub struct Solver027 {
    pub n: isize,
}

impl Default for Solver027 {
    fn default() -> Self {
        Solver027 {
            n: 1000,
        }
    }
}

impl Solver for Solver027 {
    fn solve(&self) -> isize {
        // Conjecture: a is odd negative and b is one of the 10% highest primes
        // The discriminant must be an Heegner number, in particular -163
        let (mut candidate, mut best, mut a) = (0, 0, -self.n);
        if self.n % 2 == 0 {
            a += 1;
        }
        while a < 0 {
            let (mut generator, bound) = (PrimesLessThan { n: self.n }, self.n - self.n / 10);
            let mut b = generator.next_prime();
            while b > bound {
                if square(a) - 4 * b == HEEGNER {
                    for n in 0.. {
                        if !miller_rabin(square(n) + a * n + b) {
                            break;
                        }
                        if n > best {
                            best = n;
                            candidate = a * b;
                        }
                    }
                }
                b = generator.next_prime();
            }
            a += 2;
        }
        candidate
    }
}
