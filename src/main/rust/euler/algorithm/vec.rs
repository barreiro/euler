// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::collections::HashMap;
use std::hash::Hash;

use algorithm::digits::concatenation;

#[must_use]
pub fn array_product(array: &[i64]) -> i64 {
    array.iter().product()
}

#[must_use]
pub fn array_concatenation(array: &[u64]) -> u64 {
    array.iter().fold(0, |a, &b| concatenation(b, a))
}

pub fn cluster_by<T, U>(input: &[T], f: fn(&T) -> U) -> Vec<Vec<&T>> where U: Eq + Hash {
    let mut index_map = HashMap::<_, Vec<_>>::new();
    (0..input.len()).for_each(|i| index_map.entry(f(&input[i])).or_default().push(&input[i]));
    index_map.values().cloned().collect()
}

pub fn all_unique_by<T, U>(input: &[T], f: fn(&T) -> U) -> bool where U: Eq + Hash {
    let mut index_map = HashMap::<_, Vec<_>>::new();
    (0..input.len()).for_each(|i| index_map.entry(f(&input[i])).or_default().push(&input[i]));
    index_map.len() == input.len()
}

pub fn only_unique_by<T, U>(input: &[T], f: fn(&T) -> U) -> Vec<&T> where U: Eq + Hash {
    let mut index_map = HashMap::<_, Vec<_>>::new();
    (0..input.len()).for_each(|i| index_map.entry(f(&input[i])).or_default().push(&input[i]));
    index_map.values().filter(|&v| v.len() == 1).map(|v| v[0]).collect()
}

pub fn non_unique_by<T, U>(input: &[T], f: fn(&T) -> U) -> Vec<&T> where U: Eq + Hash {
    let mut clustered = cluster_by(input, f);
    clustered.retain(|entry| entry.len() > 1);
    clustered.iter().flatten().copied().collect()
}
