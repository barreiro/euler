// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::factor::sum_of_factors;
use euler::algorithm::long::{pentagonal, int_sqrt};

/// Method for calculation the combinations of a certain number of elements in a total number of places.
/// Uses iteration instead of the formula with factorials.
pub fn choose(total: isize, elements: isize) -> isize {
    if elements <= 0 || elements >= total {
        return 0;
    }
    let (mut n, mut result) = (total, 1);
    for d in 1..=elements.min(total - elements) {
        result *= n;
        result /= d;
        n -= 1;
    }
    result
}

// --- //

/// Calculates the number of integer partition of a value, given a set of (ordered) constrains.
pub fn partition_with_constrains(value: isize, constrains: &[isize]) -> isize {
    partition_with_constrains_memoize(value as _, value as _, 0, constrains, &mut vec![vec![0; 1 + value as usize]; 1 + value as usize])
}

fn partition_with_constrains_memoize(total: usize, remaining: usize, level: usize, constrains: &[isize], cache: &mut Vec<Vec<isize>>) -> isize {
    if remaining == 0 || remaining == constrains[0] as _ {
        1
    } else if remaining < constrains[0] as _ {
        0
    } else {
        if cache[total][remaining] == 0 {
            let ceil = remaining.min(total);
            cache[total][remaining] = constrains.iter().map(|&c| c as _).take_while(|&c| c <= ceil).map(|c| partition_with_constrains_memoize(c, remaining - c, level + 1, constrains, cache)).sum();
        }
        cache[total][remaining]
    }
}

/// Calculates the number of integer partition of a value
pub fn partition(value: isize) -> isize {
    let (mut p, sum_of_factors) = (vec![0; 1 + value as usize], (0..=value).map(|v| v + sum_of_factors(v)).collect::<Vec<_>>());
    (0..=1.min(value as _)).for_each(|v| p[v] = 1);
    (2..=value as _).for_each(|v| p[v] = (0..v).map(|k| sum_of_factors[v - k] * p[k]).sum::<isize>() / v as isize);
    p[value as usize]
}

/// Finds the least number with a certain integer partition value, with a certain modulo
pub fn partition_modulo_find(modulo: isize, predicate: isize) -> isize {
    let mut cache = Vec::with_capacity(int_sqrt(modulo) as _);
    (0..2).for_each(|_| cache.push(1));
    (2..).find(|&value| partition_modulo_memoize(value, modulo, &mut cache) == predicate).unwrap()
}

// The cache is assume to be initialized with [1,1] and is to be populated by calls with incrementing value
fn partition_modulo_memoize(value: isize, modulo: isize, cache: &mut Vec<isize>) -> isize {
    let index = value as usize;
    if index >= cache.len() { cache.push(0) }
    if cache[index] == 0 {
        // uses Euler's recursive formula p(n) = p(n-1) + p(n-2) - p(n-5) - p(n-7) + p(n-12) + ... where i & 2 == 0 generates the signal sequence + + - - + + - - ...
        (0..).map(|n| if n & 1 == 0 { (n >> 1) + 1 } else { -(n >> 1) - 1 }).map(pentagonal).take_while(|&k| k <= value).enumerate().for_each(|(i, k)|
            cache[index] += if i & 2 == 0 { partition_modulo_memoize(value - k, modulo, cache) } else { -partition_modulo_memoize(value - k, modulo, cache) }
        );
        cache[index] %= modulo;
    }
    cache[index]
}

// --- //

pub struct Permutations<F, R> where F: Fn(&[isize]) -> Option<R> {
    digits: Vec<isize>,
    predicate: F,
}

pub fn permutations_of<F, R>(digits: Vec<isize>, predicate: F) -> Permutations<F, R> where F: Fn(&[isize]) -> Option<R> {
    Permutations { digits, predicate }
}

pub fn permutations_with<F, R>(start: isize, size: isize, predicate: F) -> Permutations<F, R> where F: Fn(&[isize]) -> Option<R> {
    Permutations { digits: (start..=size).collect::<Vec<_>>(), predicate }
}

impl<F, R> Iterator for Permutations<F, R> where F: Fn(&[isize]) -> Option<R> {
    type Item = R;

    fn next(&mut self) -> Option<Self::Item> {
        while self.permutate() {
            let result = (self.predicate)(&self.digits);
            if result.is_some() {
                return result;
            }
        }
        None
    }
}

impl<F, R> Permutations<F, R> where F: Fn(&[isize]) -> Option<R> {
    pub fn permutate(&mut self) -> bool {
        if self.digits.is_empty() {
            return false;
        }

        // find non-increasing suffix
        let mut i = self.digits.len() - 1;
        while i > 0 && self.digits[i - 1] >= self.digits[i] {
            i -= 1;
        }

        if i == 0 {
            return false;
        }

        // find successor to pivot
        let mut j = self.digits.len() - 1;
        while self.digits[j] <= self.digits[i - 1] {
            j -= 1;
        }

        // swap and reverse suffix
        self.digits.swap(i - 1, j);
        self.digits[i..].reverse();
        true
    }
}

// --- //

pub struct Palindromes {
    pub digits: Vec<isize>
}

pub fn palindromes() -> Palindromes {
    Palindromes { digits: vec![0] }
}

impl Iterator for Palindromes {
    type Item = Vec<isize>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.palindrome_increase() && !self.palindrome_rotate() {
            self.palindrome_expand();
        }
        Some(self.digits.to_vec())
    }
}

impl Palindromes {
    fn palindrome_increase(&mut self) -> bool {
        let middle = self.digits.len() / 2;
        if self.digits[middle] >= 9 {
            return false;
        }
        if self.digits.len() % 2 == 0 {
            self.digits[middle - 1] += 1;
        }
        self.digits[middle] += 1;
        true
    }

    fn palindrome_rotate(&mut self) -> bool {
        let size = self.digits.len();
        for i in size / 2 + 1..size {
            self.digits[i - 1] = 0;
            self.digits[size - i] = 0;

            if self.digits[i] != 9 {
                self.digits[i] += 1;
                self.digits[size - i - 1] += 1;
                return true;
            }
        }
        false
    }

    fn palindrome_expand(&mut self) {
        let size = self.digits.len();
        self.digits = vec![0; size + 1];
        self.digits[0] = 1;
        self.digits[size] = 1;
    }
}
