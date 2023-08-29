// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::cast::Cast;
use algorithm::digits::{concatenation, Digits};
use algorithm::root::{ceil_sqrt_u64, int_log_10, pow_10};
use Solver;

/// Take the number `192` and multiply it by each of `1, 2, and 3`:
/// ```
/// 192 × 1 = 192
/// 192 × 2 = 384
/// 192 × 3 = 576
/// ```
/// By concatenating each product we get the `1` to `9` pandigital, `192384576`. We will call `192384576` the concatenated product of `192` and `(1,2,3)`
///
/// The same can be achieved by starting with `9` and multiplying by `1, 2, 3, 4, and 5`, giving the pandigital, `918273645`, which is the concatenated product of `9` and `(1,2,3,4,5)`.
///
/// What is the largest `1` to `9` pandigital `9-digit` number that can be formed as the concatenated product of an integer with `(1,2, ... , n)` where `n > 1`?
pub struct Solver038 {
    pub n: u64,
}

impl Default for Solver038 {
    fn default() -> Self {
        Self { n: 9 }
    }
}

impl Solver for Solver038 {
    fn problem_name(&self) -> &str { "Pandigital multiples" }

    fn solve(&self) -> i64 {
        let natural_product_concatenation = |value, size| {
            let (mut n, mut result) = (value, value);
            loop {
                if int_log_10(result) >= size {
                    return Digits::from(result);
                }
                n += value;
                result = concatenation(result, n);
            }
        };

        // generate the natural product concatenation and check if it's a pandigital. find the first according to the most significant digit ordering
        decrementing_by_most_significant(self.n, ceil_sqrt_u64(self.n)).map(|v| natural_product_concatenation(v, self.n)).filter(|d| d.len() == self.n.as_usize()).find(Digits::is_pandigital).as_i64()
    }
}

// --- //

/// creates an iterator that returns elements ordered by most significant digit: 999 ... 900 -> 99 ... 90 -> 9 -> 899 ... 800 -> 89 .. 80 -> 8 -> 799
fn decrementing_by_most_significant(value: u64, scale: u64) -> impl Iterator<Item=u64> {
    DecrementingByMostSignificant { range: Box::new(0..0), size: 0, value: value + 1, scale }
}

struct DecrementingByMostSignificant {
    range: Box<dyn Iterator<Item=u64>>,
    size: u64,
    value: u64,
    scale: u64,
}

impl Iterator for DecrementingByMostSignificant {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        self.range.next().or_else(|| {
            if self.size > 0 {
                self.size -= 1;
            } else {
                self.size = self.scale;
                self.value -= 1;
            }
            if self.value > 0 {
                self.range = Box::new((self.value * pow_10(self.size)..(self.value + 1) * pow_10(self.size)).rev());
                self.range.next().filter(|&v| v > 1)
            } else {
                None
            }
        })
    }
}
