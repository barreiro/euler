// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::iter::{from_fn, once};
use std::ops::{Add, Sub};

use crate::algorithm::cast::Cast;
use crate::algorithm::digits::Digit;
use crate::algorithm::factor::sum_of_factors;
use crate::algorithm::filter::less_or_equal_than;
use crate::algorithm::long::{are_coprime, IncrementAndGet, pentagonal};
use crate::algorithm::root::ceil_sqrt_u64;

/// method for calculation the combinations of a certain number of elements in a total number of places.
/// uses iteration instead of the formula with factorials.
#[must_use]
pub fn choose(total: u64, elements: u64) -> u64 {
    if elements == 0 || elements >= total {
        return 1;
    }
    let (mut n, mut result) = (total, 1);
    for d in 1..=elements.min(total - elements) {
        // result *= n;
        // result /= d;

        // split result * n / d into (result / d * d + result % d) * n / d
        result = result / d * n + result % d * n / d;
        n -= 1;
    }
    result
}

/// calculate the number of ways to create subsets with repeating elements
#[must_use]
pub fn multi_choose(total: u64, elements: u64) -> u64 {
    choose(total + elements - 1, elements)
}

// --- //

/// calculates the number of integer partitions of a value, given a set of (ordered) constrains.
#[must_use]
pub fn partition_with_constrains(value: u64, constrains: &[u64]) -> u64 {
    partition_with_constrains_memoize(value, value, constrains, &mut vec![vec![None; value.as_usize() + 1]; value.as_usize() + 1])
}

fn partition_with_constrains_memoize(total: u64, remaining: u64, constrains: &[u64], cache: &mut Vec<Vec<Option<u64>>>) -> u64 {
    if remaining == 0 || remaining == constrains[0] {
        1
    } else if remaining < constrains[0] {
        0
    } else {
        let (i, j) = (total.as_usize(), remaining.as_usize());
        if cache[i][j].is_none() {
            let ceil = remaining.min(total);
            cache[i][j] = Some(constrains.iter().take_while(|&&c| c <= ceil).map(|&c| partition_with_constrains_memoize(c, remaining - c, constrains, cache)).sum());
        }
        cache[i][j].expect("Cache should be populated")
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
    let mut cache = Vec::with_capacity(ceil_sqrt_u64(modulo).as_usize());
    cache.extend_from_slice(&[1, 1]);
    (2..).find(|&value| partition_modulo_memoize(value, modulo, &mut cache) == predicate).as_u64()
}

// cache assumed to be initialized with [1,1] and is to be populated by calls with incrementing value
#[allow(clippy::maybe_infinite_iter)]
fn partition_modulo_memoize(value: u64, modulo: u64, cache: &mut Vec<u64>) -> u64 {
    if let Some(&partition_modulo) = cache.get(value.as_usize()) {
        partition_modulo
    } else {
        // uses Euler's recursive formula p(n) = p(n-1) + p(n-2) - p(n-5) - p(n-7) + p(n-12) + ... where i & 2 == 0 generates the signal sequence + + - - + + - - ...
        let partition_modulo = (0..).map(|n| if n & 1 == 0 { (n >> 1) + 1 } else { -(n >> 1) - 1 }).map(pentagonal).take_while(less_or_equal_than(value.as_i64())).enumerate().fold(0, |pm, (i, k)| {
            (if i & 2 == 0 { Add::add } else { Sub::sub })(pm, partition_modulo_memoize(value - k.as_u64(), modulo, cache).as_i64())
        }).rem_euclid(modulo.as_i64()).as_u64();
        cache.push(partition_modulo);
        partition_modulo
    }
}

// --- //

/// provides an iterator of the permutations of the given elements. requires the elements to be sorted to provide all possible permutations
pub fn permutations_of_set<T>(elements: Vec<T>) -> impl Iterator<Item=Vec<T>> where T: PartialOrd + Copy {
    permutations_of_set_with(elements, |p| Some(p.to_vec()))
}

/// provides an iterator of the permutations of the given elements that satisfy a given mapping predicate. requires the elements to be sorted to provide all possible permutations
pub fn permutations_of_set_with<T, F, R>(elements: Vec<T>, predicate: F) -> impl Iterator<Item=R> where T: PartialOrd, F: FnMut(&[T]) -> Option<R> {
    Permutations { elements, predicate }
}

/// provides an iterator of permutations of the digits `start` and `size` (inclusive) that satisfy a given mapping predicate.
pub fn permutations_of_digits_with<F, R>(start: Digit, size: Digit, predicate: F) -> impl Iterator<Item=R> where F: FnMut(&[Digit]) -> Option<R> {
    permutations_of_set_with((start..=size).collect::<Vec<_>>(), predicate)
}

struct Permutations<T, F, R> where T: PartialOrd, F: FnMut(&[T]) -> Option<R> {
    elements: Vec<T>,
    predicate: F,
}

impl<T, F, R> Iterator for Permutations<T, F, R> where T: PartialOrd, F: FnMut(&[T]) -> Option<R> {
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

impl<T, F, R> Permutations<T, F, R> where T: PartialOrd, F: FnMut(&[T]) -> Option<R> {
    pub fn permutate(&mut self) -> bool {
        // find non-increasing suffix
        (1..self.elements.len()).rev().find(|&i| self.elements[i - 1] < self.elements[i]).map_or(false, |index| {
            // swap with a successor and reverse suffix
            let pivot = (0..self.elements.len()).rev().find(|&j| self.elements[j] > self.elements[index - 1]).expect("There should be an element to swap");
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
    from_fn(move || (!indexes.is_empty() && indexes[0] < elements.len()).then(|| {
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

/// provides an iterator of partitions of the elements
/// # Panics
/// Will panic if there are no partitions
pub fn partitions_of_set<T>(elements: &[T]) -> impl Iterator<Item=Vec<Vec<T>>> + '_ where T: Copy {
    // following the paper "Efficient Generation of Set Partitions" by Michael Orlov
    let (mut k, mut m) = (vec![0; elements.len()], vec![0; elements.len()]);
    once(vec![elements.to_vec()]).chain(from_fn(move || {
        (1..elements.len()).rev().find_map(|i| (k[i] <= m[i - 1]).then(|| {
            m[i] = m[i].max(k[i].increment_and_get());
            (i + 1..elements.len()).for_each(|j| (k[j], m[j]) = (k[0], m[i]));

            // `k[e]` are the partition indexes for `elements[e]`
            let mut result = vec![vec![]; *k.iter().max().expect("A partition should exist") + 1];
            (0..elements.len()).for_each(|e| result[k[e]].push(elements[e]));
            result
        }))
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
            let result = predicate(&pattern.iter().rev().map(|&i| elements[i]).collect::<Vec<_>>());
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
        // m and n have opposite parity – i.e., if one is odd, the other must be even.
        // m and n are coprime – i.e., they have no common integer factors greater than 1.
        while {
            n += 2;
            if n >= m {
                n = if m.increment_and_get() % 2 == 0 { 1 } else { 2 };
            }
            !are_coprime(m, n)
        } {}

        // uses Euclides Formula --- a=m^2-n^2 --- b=2nm --- c=m^2+n^2 --- with m>n
        let (m_square, n_square) = (m * m, n * n);
        let (side_a, side_b, c) = (m_square - n_square, 2 * m * n, m_square + n_square);
        Some((side_a.min(side_b), side_a.max(side_b), c))
    })
}
