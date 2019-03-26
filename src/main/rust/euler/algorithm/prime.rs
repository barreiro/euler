// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::collections::HashMap;

use euler::algorithm::long::int_sqrt;
use euler::algorithm::long::power_modulo;

const MILLER_RABIN_THRESHOLD: isize = 4759123141;
const MILLER_RABIN_FAST: &[isize] = &[2, 7, 61];
const MILLER_RABIN_BASE: &[isize] = &[2, 325, 9375, 28178, 450775, 9780504, 1795265022];

/// calculates the prime factors of a given number. The result is a map where the keys are primes and the values are the occurrences
pub fn prime_factors(n: isize) -> HashMap<isize, isize> {
    let (mut factor_map, mut generator, mut value, small, stop) = (HashMap::new(), GeneratorTrialDivision::default(), n, n <= i32::max_value() as isize, int_sqrt(n));
    loop {
        let factor = generator.next_prime();
        while if small { value as i32 % factor as i32 == 0 } else { value % factor == 0 } {
            value /= factor;
            *factor_map.entry(factor).or_insert(0) += 1;
        }
        if factor >= stop {
            // if the number is prime, or if there is still a remainder, add itself as a factor
            if value >= factor || factor_map.is_empty() {
                factor_map.insert(value, 1);
            }
            break;
        }
    }
    factor_map
}

/// Trait for prime generators
pub trait PrimeGenerator {
    fn next_prime(&mut self) -> isize;
}

/// closure that generates primes based on the method of trial division
pub struct GeneratorTrialDivision {
    cache: Option<Vec<isize>>
}

impl Default for GeneratorTrialDivision {
    fn default() -> Self {
        GeneratorTrialDivision { cache: None }
    }
}

impl PrimeGenerator for GeneratorTrialDivision {
    fn next_prime(&mut self) -> isize {
        let next: isize = match &self.cache {
            None => {
                2
            }
            Some(cache) => match cache.last() {
                None => {
                    3
                }
                Some(last) => {
                    let (mut found, mut candidate, stop) = (false, *last, &int_sqrt(*last));
                    loop {
                        candidate += 2;

                        let (mut i, small, len, cache_ptr) = (0 as isize, candidate <= i32::max_value() as isize, cache.len() as isize, cache.as_ptr());

                        while i < len {
                            // fast read of cache
                            let factor = unsafe { *cache_ptr.offset(i) as isize };
                            i += 1;

                            if if small { candidate as i32 % factor as i32 == 0 } else { candidate % factor == 0 } {
                                break;
                            }

                            if factor >= *stop {
                                found = true;
                                break;
                            }
                        }
                        if found {
                            break candidate;
                        }
                    }
                }
            }
        };
        match next {
            2 => { self.cache = Some(vec![]) }
            _ => { self.cache.get_or_insert(vec![]).push(next) }
        }
        next
    }
}

/// closure that generates primes prime numbers, starting with the one below N.
pub struct PrimesLessThan {
    pub n: isize
}

impl PrimeGenerator for PrimesLessThan {
    fn next_prime(&mut self) -> isize {
        self.n -= 1;
        while !miller_rabin(self.n) {
            self.n -= 1;
        }
        self.n
    }
}

pub fn miller_rabin(n: isize) -> bool {
    let base = if n < MILLER_RABIN_THRESHOLD { MILLER_RABIN_FAST } else { MILLER_RABIN_BASE };
    for b in base {
        if n > *b && !miller_rabin_pass(*b, n) {
            return false;
        }
    }
    true
}

fn miller_rabin_pass(b: isize, n: isize) -> bool {
    let s = (n - 1).trailing_zeros() as isize;
    let d = (n - 1) >> s;
    let mut a = power_modulo(b, d, n);

    if a == 1 {
        return true
    }
    for _ in 0..s - 1 {
        if a == n - 1 {
            return true;
        }
        if a == 1 {
            return false;
        }
        a = power_modulo(a, 2, n);
    }
    a == n - 1
}
