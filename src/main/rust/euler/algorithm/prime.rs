// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::collections::HashMap;

use euler::algorithm::long::{floor_sqrt, is_even};
use euler::algorithm::long::power_modulo;

// Bases for the Miller-Rabin test
const MR_THRESHOLD: isize = 4_759_123_141;
const MR_BASE_FAST: &[isize] = &[2, 7, 61];
const MR_BASE: &[isize] = &[2, 325, 9_375, 28_178, 450_775, 9_780_504, 1_795_265_022];

/// calculates the prime factors of a given number. The result is a map where the keys are primes and the values are the occurrences
pub fn prime_factors(n: isize) -> HashMap<isize, isize> {
    let (mut factor_map, mut value, small, stop) = (HashMap::new(), n, n <= i32::max_value() as _, floor_sqrt(n));
    for factor in generator_trial_division() {
        while if small { value as i32 % factor as i32 == 0 } else { value % factor == 0 } {
            value = if small { (value as i32 / factor as i32) as _ } else { value / factor };
            factor_map.entry(factor).and_modify(|e| *e += 1).or_insert(1);
        }
        if value == 1 {
            break;
        }
        if factor >= stop {
            // the number is prime or if there is still a remainder, add it as a factor
            factor_map.insert(value, 1);
            break;
        }
    }
    factor_map
}

// --- //

/// closure that generates primes based on the method of trial division
pub struct GeneratorTrialDivision {
    sieve: Vec<isize>
}

pub fn generator_trial_division() -> GeneratorTrialDivision {
    GeneratorTrialDivision { sieve: vec![] }
}

impl Iterator for GeneratorTrialDivision {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        let next = match self.sieve.last() {
            None => { 2 }
            Some(2) => {
                self.sieve.clear();
                3
            }
            last => { (last.unwrap() + 2..).step_by(2).find(|&candidate| prime_sieve(candidate, &self.sieve)).unwrap() }
        };

        self.sieve.push(next);
        Some(next)
    }
}

pub fn prime_sieve(n: isize, sieve: &[isize]) -> bool {
    let ceil = floor_sqrt(n);
    if n <= i32::max_value() as _ {
        !sieve.iter().take_while(|&&factor| factor <= ceil).any(|&f| n as i32 % f as i32 == 0)
    } else {
        !sieve.iter().take_while(|&&factor| factor <= ceil).any(|&f| n % f == 0)
    }
}

// --- //

/// closure that generates primes based on the method of wheel factorization
pub struct GeneratorWheel {
    size: isize,
    increments: Vec<isize>,
    sieve: Vec<isize>,
}

pub fn generator_wheel() -> GeneratorWheel {
    generator_custom_wheel(&[2, 3, 5, 7, 11])
}

pub fn generator_custom_wheel(primes: &[isize]) -> GeneratorWheel {
    let size = primes.iter().product();
    let buckets = (size..=1 + size * 2).filter(|n| primes.iter().all(|p| n % p != 0)).map(|n| n - size).collect::<Vec<_>>();
    let mut increments = vec![0; *buckets.last().unwrap() as usize];
    (0..buckets.len() - 1).for_each(|i| increments[buckets[i] as usize] = buckets[i + 1] - buckets[i]);
    GeneratorWheel { size, increments, sieve: vec![] }
}

impl Iterator for GeneratorWheel {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        let next = match self.sieve.last() {
            None => { 2 }
            Some(2) => {
                self.sieve.clear();
                3
            }
            last => {
                if *last.unwrap() < self.size {
                    (last.unwrap() + 2..).step_by(2).find(|&candidate| prime_sieve(candidate, &self.sieve)).unwrap()
                } else {
                    let (mut candidate, mut index) = (*last.unwrap(), (last.unwrap() % self.size));
                    loop {
                        let increment = self.increments[index as usize];
                        candidate += increment;
                        if prime_sieve(candidate, &self.sieve) {
                            break candidate;
                        }
                        index += increment;
                        if index > self.size { index = 1 }
                    }
                }
            }
        };

        self.sieve.push(next);
        Some(next)
    }
}

// --- //

/// closure that generates primes prime numbers, starting with the one below N.
pub struct PrimesLessThan {
    pub n: isize
}

pub fn primes_less_than(n: isize) -> PrimesLessThan {
    PrimesLessThan { n: if is_even(n) { n - 1 } else { n } }
}

impl Iterator for PrimesLessThan {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.n -= 2;
            if miller_rabin(self.n) {
                return if self.n > 1 { Some(self.n) } else { None };
            }
        }
    }
}

// --- //

/// closure that generates primorals, the product of the smallest primes
pub struct Primoral {
    generator: GeneratorTrialDivision,
    product: isize
}

pub fn primoral() -> Primoral {
    Primoral { generator: generator_trial_division(), product: 1 }
}

impl Iterator for Primoral {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        self.product *= self.generator.next().unwrap();
        Some(self.product)
    }
}

// --- //

pub fn miller_rabin(value: isize) -> bool {
    value != 1 && if value < MR_THRESHOLD { MR_BASE_FAST } else { MR_BASE }.iter().all(|&b| value <= b || miller_rabin_pass(b, value))
}

fn miller_rabin_pass(b: isize, value: isize) -> bool {
    let s = (value - 1).trailing_zeros() as isize;
    let d = (value - 1) >> s;
    let mut a = power_modulo(b, d, value);

    if a == 1 {
        return true;
    }
    for _ in 0..s - 1 {
        if a == value - 1 {
            return true;
        }
        if a == 1 {
            return false;
        }
        a = power_modulo(a, 2, value);
    }
    a == value - 1
}
