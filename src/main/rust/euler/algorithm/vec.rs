// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use crate::algorithm::digits::concatenation;

#[must_use]
pub fn array_sum(array: &[i64]) -> i64 {
    array.iter().sum()
}

#[must_use]
pub fn array_sum_u64(array: &[u64]) -> u64 {
    array.iter().sum()
}

#[must_use]
pub fn array_product(array: &[i64]) -> i64 {
    array.iter().product()
}

#[must_use]
pub fn array_product_u64(array: &[u64]) -> u64 {
    array.iter().product()
}

#[must_use]
pub fn array_concatenation(array: &[u64]) -> u64 {
    array.iter().fold(0, |a, &b| concatenation(b, a))
}

#[must_use]
pub fn array_max<T>(array: &[T]) -> &T where T: Ord {
    array.iter().max().expect("Array should not be empty")
}

#[must_use]
pub fn array_min<T>(array: &[T]) -> &T where T: Ord {
    array.iter().min().expect("Array should not be empty")
}

// --- //

pub fn cluster_by<T, U>(input: &[T], f: fn(&T) -> U) -> Vec<Vec<&T>> where U: Eq + Hash {
    let mut index_map = HashMap::<_, Vec<_>>::new();
    (0..input.len()).for_each(|i| index_map.entry(f(&input[i])).or_default().push(&input[i]));
    index_map.values().cloned().collect()
}

pub fn all_unique<T>(input: &[T]) -> bool where T: Eq + Hash {
    let mut set = HashSet::<_>::new();
    (0..input.len()).all(|i| set.insert(&input[i]))
}

pub fn all_unique_by<T, U>(input: &[T], f: fn(&T) -> U) -> bool where U: Eq + Hash {
    let mut set = HashSet::<_>::new();
    (0..input.len()).all(|i| set.insert(f(&input[i])))
}

pub fn only_unique_by<T, U>(input: &[T], f: fn(&T) -> U) -> Vec<&T> where U: Eq + Hash {
    let mut index_map = HashMap::<_, Vec<_>>::new();
    (0..input.len()).for_each(|i| index_map.entry(f(&input[i])).or_default().push(&input[i]));
    index_map.values().filter_map(|v| v.first().map(ToOwned::to_owned).filter(|_| v.len() == 1)).collect()
}

pub fn non_unique_by<T, U>(input: &[T], f: fn(&T) -> U) -> Vec<&T> where U: Eq + Hash {
    let mut clustered = cluster_by(input, f);
    clustered.retain(|entry| entry.len() > 1);
    clustered.iter().flatten().copied().collect()
}
