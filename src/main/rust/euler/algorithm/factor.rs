// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::collections::HashMap;

use euler::algorithm::long::{int_sqrt, pow, square};

pub fn factor_composition(factor_map: HashMap<isize, isize>) -> isize {
    factor_map.iter().map(|(base, exp)| -> isize { pow(*base, *exp) }).product()
}

pub fn has_factor_below(value: isize, roof: isize) -> bool {
    for l in int_sqrt(value)..roof {
        if value % l == 0 && value / l < roof {
            return true;
        }
    }
    false
}

pub fn number_of_factors(value: isize) -> isize {
    let (mut factors, mut i, small, perfect) = (0, int_sqrt(value), value <= std::i32::MAX as isize, square(int_sqrt(value)) == value);
    while i > 0 {
        if if small { value as i32 % i as i32 == 0 } else { value % i == 0 } {
            factors += 1
        }
        i -= 1;
    }
    // need to adjust the number of divisors if the number is a perfect square
    if perfect { 2 * factors - 1 } else { 2 * factors }
}

// defined according to problem 21: numbers less than n which divide evenly into n
pub fn sum_of_factors(value: isize) -> isize {
    let (mut sum, ceiling, small, perfect) = (1, int_sqrt(value), value <= std::i32::MAX as isize, square(int_sqrt(value)) == value);
    for i in (2..ceiling + 1).rev() {
        if if small { value as i32 % i as i32 == 0 } else { value % i == 0 } {
            sum += i + value / i;
        }
    }

    // need to adjust the number of divisors if the number is a perfect square
    if perfect { sum - ceiling } else { sum }
}

pub fn is_abundant(value : isize) -> bool {
    value < sum_of_factors(value)
}