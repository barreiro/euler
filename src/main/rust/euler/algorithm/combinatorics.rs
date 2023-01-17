// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::iter::from_fn;
use std::ops::{Add, Sub};

use algorithm::cast::Cast;
use algorithm::factor::sum_of_factors;
use algorithm::filter::less_or_equal_than;
use algorithm::long::{are_coprime, pentagonal};
use algorithm::root::floor_sqrt_u64;

/// method for calculation the combinations of a certain number of elements in a total number of places.
/// uses iteration instead of the formula with factorials.
#[must_use]
pub fn choose(total: u64, elements: u64) -> u64 {
    if elements == 0 || elements >= total {
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

/// calculates the number of integer partitions of a value, given a set of (ordered) constrains.
#[must_use]
pub fn partition_with_constrains(value: u64, constrains: &[u64]) -> u64 {
    partition_with_constrains_memoize(value, value, constrains, &mut vec![vec![0; value.as_usize() + 1]; value.as_usize() + 1])
}

fn partition_with_constrains_memoize(total: u64, remaining: u64, constrains: &[u64], cache: &mut Vec<Vec<u64>>) -> u64 {
    if remaining == 0 || remaining == constrains[0] {
        1
    } else if remaining < constrains[0] {
        0
    } else {
        let (i, j) = (total.as_usize(), remaining.as_usize());
        if cache[i][j] == 0 {
            let ceil = remaining.min(total);
            cache[i][j] = constrains.iter().map(|&c| c as _).take_while(|&c| c <= ceil).map(|c| partition_with_constrains_memoize(c, remaining - c, constrains, cache)).sum();
        }
        cache[i][j]
    }
}

/// calculates the number of integer partitions of a value
#[must_use]
pub fn partition(value: u64) -> u64 {
    let (mut p, sum_of_factors) = (vec![0; value.as_usize() + 1], (0..=value.as_i64()).map(|v| v + sum_of_factors(v)).collect::<Vec<_>>());
    (0..=1.min(value.as_usize())).for_each(|v| p[v] = 1);
    (2..=value.as_usize()).for_each(|v| p[v] = (0..v).map(|k| sum_of_factors[v - k] * p[k]).sum::<i64>() / v.as_i64());
    p[value.as_usize()].as_u64()
}

/// finds the least number with a certain integer partition value, with a certain modulo
#[must_use]
#[allow(clippy::maybe_infinite_iter)]
pub fn partition_modulo_find(modulo: u64, predicate: u64) -> u64 {
    let mut cache = Vec::with_capacity(1 + floor_sqrt_u64(modulo).as_usize());
    cache.extend_from_slice(&[1, 1]);
    (2..).find(|&value| partition_modulo_memoize(value, modulo, &mut cache) == predicate).as_u64()
}

// the cache is assume to be initialized with [1,1] and is to be populated by calls with incrementing value
#[allow(clippy::maybe_infinite_iter)]
fn partition_modulo_memoize(value: u64, modulo: u64, cache: &mut Vec<u64>) -> u64 {
    let index = value.as_usize();
    if index < cache.len() {
        return cache[index];
    }
    // uses Euler's recursive formula p(n) = p(n-1) + p(n-2) - p(n-5) - p(n-7) + p(n-12) + ... where i & 2 == 0 generates the signal sequence + + - - + + - - ...
    let partition_modulo = (0..).map(|n| if n & 1 == 0 { (n >> 1) + 1 } else { -(n >> 1) - 1 }).map(pentagonal).take_while(less_or_equal_than(value.as_i64())).enumerate().fold(0, |pm, (i, k)| {
        (if i & 2 == 0 { Add::add } else { Sub::sub })(pm, partition_modulo_memoize(value - k.as_u64(), modulo, cache).as_i64())
    });
    cache.push(partition_modulo.rem_euclid(modulo.as_i64()).as_u64());
    cache[index]
}

// --- //

/// provides an iterator of the permutations of the given elements. requires the elements to be sorted in order to provide all possible permutations
pub fn permutations_of_set<T>(elements: Vec<T>) -> impl Iterator<Item=Vec<T>> where T: PartialOrd + Copy {
    permutations_of_set_with(elements, |p| Some(p.to_vec()))
}

/// provides an iterator of the permutations of the given elements that satisfy a given mapping predicate. requires the elements to be sorted in order to provide all possible permutations
pub fn permutations_of_set_with<T, F, R>(elements: Vec<T>, predicate: F) -> impl Iterator<Item=R> where T: PartialOrd, F: Fn(&[T]) -> Option<R> {
    Permutations { elements, predicate }
}

/// provides an iterator of permutations of the digits `start` and `size` (inclusive) that satisfy a given mapping predicate.
pub fn permutations_of_digits_with<F, R>(start: u8, size: u8, predicate: F) -> impl Iterator<Item=R> where F: Fn(&[u8]) -> Option<R> {
    permutations_of_set_with((start..=size).collect::<Vec<_>>(), predicate)
}

struct Permutations<T, F, R> where T: PartialOrd, F: Fn(&[T]) -> Option<R> {
    elements: Vec<T>,
    predicate: F,
}

impl<T, F, R> Iterator for Permutations<T, F, R> where T: PartialOrd, F: Fn(&[T]) -> Option<R> {
    type Item = R;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.elements.is_empty() {
            let result = (self.predicate)(&self.elements);
            (!self.permutate()).then(|| self.elements.clear());
            if result.is_some() {
                return result;
            }
        }
        None
    }
}

impl<T, F, R> Permutations<T, F, R> where T: PartialOrd, F: Fn(&[T]) -> Option<R> {
    pub fn permutate(&mut self) -> bool {
        // find non-increasing suffix
        (1..self.elements.len()).rev().find(|&i| self.elements[i - 1] < self.elements[i]).map_or(false, |index| {
            // swap with a successor and reverse suffix
            let pivot = (0..self.elements.len()).rev().find(|&j| self.elements[j] > self.elements[index - 1]).unwrap();
            self.elements.swap(index - 1, pivot);
            self.elements[index..].reverse();
            true
        })
    }
}

// --- //

/// provides an iterator of combinations of the elements (with repetition of elements)
pub fn permutations_with_repetition_of_set<T>(elements: Vec<T>, size: usize) -> impl Iterator<Item=Vec<T>> where T: Copy {
    let mut indexes = vec![0; size];
    from_fn(move || (indexes[0] >= elements.len()).then(|| {
        let mut result = Vec::with_capacity(indexes.len());
        indexes.iter().for_each(|&i| result.push(elements[i]));
        for i in (0..indexes.len()).rev() {
            indexes[i] += 1;
            if i != 0 && indexes[i] >= elements.len() {
                indexes[i] = 0;
            } else {
                break;
            }
        }
        result
    }))
}

// --- //

/// provides an iterator of combinations of the given elements
pub fn combinations<T>(elements: Vec<T>, size: usize) -> impl Iterator<Item=Vec<T>> where T: PartialOrd + Copy {
    combinations_with(elements, size, |p| Some(p.to_vec()))
}

/// provides an iterator of combinations of elements that satisfy a given mapping predicate
pub fn combinations_with<T, F, R>(elements: Vec<T>, size: usize, predicate: F) -> impl Iterator<Item=R> where T: Copy, F: Fn(&[T]) -> Option<R> {
    // this implementation uses a vec of positions. it can be improved for elements.len() < isize::BITS using a trick known as Gospher's Hack
    let (mut pattern, target) = ((0..size).rev().collect::<Vec<_>>(), elements.len() - 1);
    let increase = move |pattern: &mut Vec<usize>| {
        for i in 0..pattern.len() {
            if pattern[i] + i < target {
                pattern[i] += 1;
                for j in 0..i {
                    pattern[j] = pattern[i] + i - j;
                }
                return;
            }
        }
        pattern.clear();
    };

    from_fn(move || {
        while !pattern.is_empty() {
            let result = (predicate)(&pattern.iter().rev().map(|&i| elements[i]).collect::<Vec<_>>());
            increase(&mut pattern);
            if result.is_some() {
                return result;
            }
        }
        None
    })
}

// --- //

/// iteration over all Pythagorean triples `(a < b < c)` (but each can have smaller values later on the iteration)
/// bound by `a < square(bound)`
pub fn pythagorean_triplets_lower_bound(bound: u64) -> impl Iterator<Item=(u64, u64, u64)> {
    pythagorean_triplets().take_while(move |&(a, b, c)| c - b != 2 && a < bound * bound)
}

/// iteration over all Pythagorean triples `(a < b < c)` (but each can have smaller values later on the iteration)
pub fn pythagorean_triplets() -> impl Iterator<Item=(u64, u64, u64)> {
    let (mut m, mut n) = (1, 0);
    from_fn(move || {
        // a primitive Pythagorean triple additionally requires:
        // m and n have opposite parity – i.e. if one is odd, the other must be even.
        // m and n are coprime – i.e. they have no common integer factors greater than 1.
        while {
            n += 2;
            if n >= m {
                m += 1;
                n = if m % 2 == 0 { 1 } else { 2 };
            }
            !are_coprime(m.as_i64(), n.as_i64())
        } {}

        // uses Euclides Formula --- a=m^2-n^2 --- b=2nm --- c=m^2+n^2 --- with m>n
        let (m_square, n_square) = (m * m, n * n);
        let (side_a, side_b, c) = (m_square - n_square, 2 * m * n, m_square + n_square);
        Some((side_a.min(side_b), side_a.max(side_b), c))
    })
}
