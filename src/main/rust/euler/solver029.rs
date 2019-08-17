// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::bit::bit_set;
use euler::algorithm::long::{int_sqrt, pow, square};
use euler::algorithm::prime::prime_factors;
use euler::Solver;

// Consider all integer combinations of a^b for 2 ≤ a ≤ 5 and 2 ≤ b ≤ 5:
// 2^2=4,  2^3=8,   2^4=16,  2^5=32
// 3^2=9,  3^3=27,  3^4=81,  3^5=243
// 4^2=16, 4^3=64,  4^4=256, 4^5=1024
// 5^2=25, 5^3=125, 5^4=625, 5^5=3125
//
// If they are then placed in numerical order, with any repeats removed, we get the following sequence of 15 distinct terms: 4, 8, 9, 16, 25, 27, 32, 64, 81, 125, 243, 256, 625, 1024, 3125
// How many distinct terms are in the sequence generated by a^b for 2 ≤ a ≤ 100 and 2 ≤ b ≤ 100?

pub struct Solver029 {
    pub n: isize,
}

impl Default for Solver029 {
    fn default() -> Self {
        Solver029 { n: 100 }
    }
}

impl Solver for Solver029 {
    fn solve(&self) -> isize {
        // a given number 2≤a≤N can only produce duplicates if 'a^n' can be expressed as '(b^i)^j' with n=i∗j and 2≤n,i,j≤N
        let (mut duplicates, bound, mut unique) = (0, int_sqrt(self.n) + 1, bit_set());
        for b in 2..bound {
            for i in 2..bound {
                let a = pow(b, i);
                if a <= self.n && unique.insert(a) {
                    duplicates += (self.n / i) - 1;

                    // the trivial duplicates that have i*j<=N are accounted. there may be an equivalent of the factorization that still satisfies the relation
                    for j in 1 + self.n / i..self.n {
                        let (factored_base, factored_exp) = factored_power(a, j);
                        let (mut base, mut exp, mut k) = (factored_base, factored_exp, 2);
                        while base < a {
                            if exp <= self.n && factored_exp % exp == 0 {
                                duplicates += 1;
                                break;
                            }
                            base *= factored_base;
                            exp = factored_exp / k;
                            k += 1;
                        }
                    }
                }
            }
        }
        square(self.n - 1) - duplicates
    }
}

fn factored_power(base: isize, power: isize) -> (isize, isize) {
    let (mut factored_base, mut factored_exp, factors) = (1, 0, prime_factors(base));
    for (&k, &v) in &factors {
        factored_base *= k;
        factored_exp += v;
    }
    (factored_base, factored_exp * power / factors.len() as isize)
}
