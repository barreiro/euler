// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::cmp::min;

use euler::algorithm::long::to_digits;

/// Method for calculation the combinations of a certain number of elements in a total number of places.
/// Uses recursion instead of the formula with factorials.
pub fn choose(total: isize, elements: isize) -> isize {
    // Take full advantage of symmetry
    let min_elements = elements.min(total - elements);
    if min_elements < 0 {
        return 0;
    }
    if min_elements == 0 {
        return 1;
    }
    choose_memoize(total as usize, min_elements as usize, vec![vec![0; min_elements as usize + 1]; total as usize + 1].as_mut())
}

pub fn choose_memoize(total: usize, elements: usize, cache: &mut Vec<Vec<isize>>) -> isize {
    let min_elements = elements.min(total - elements);
    if min_elements == 1 {
        return total as isize;
    }
    if cache.as_slice()[total].as_slice()[min_elements] != 0 {
        return cache[total][min_elements];
    }
    let value = choose_memoize(total - 1, min_elements - 1, cache) + choose_memoize(total - 1, min_elements, cache);
    cache.as_mut_slice()[total].as_mut_slice()[min_elements] = value;
    return value;
}

// --- //

/// Calculates the number of integer partition of a value, given a set of constrains.
pub fn partition(value: isize, constrains: &[isize]) -> isize {
    let mut cache = vec![vec![0isize; 1 + value as usize]; 1 + value as usize];

    partition_memoize(value, value, 0, constrains, &mut cache)
}

fn partition_memoize(remaining: isize, total: isize, sum: isize, constrains: &[isize], cache: &mut Vec<Vec<isize>>) -> isize {
    if remaining == 0 {
        return 1;
    }
    if cache[remaining as usize][total as usize] != 0 {
        return cache[remaining as usize][total as usize];
    }
    let (mut l, bound) = (0, min(remaining, total));
    for c in constrains {
        if *c <= bound {
            l += partition_memoize(remaining - *c, *c, sum + *c, constrains, cache);
        } else {
            break; // assuming constraints are ordered
        }
    }
    cache[remaining as usize][total as usize] = l;
    l
}

// --- //

pub struct PermutationArray {
    array: Vec<isize>
}

pub fn permutation_array(array: Vec<isize>) -> PermutationArray {
    PermutationArray { array }
}

impl Iterator for PermutationArray {
    type Item = Vec<isize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.array.is_empty() {
            return None;
        }

        // find non-increasing suffix
        let mut i = self.array.len() - 1;
        while i > 0 && self.array[i - 1] >= self.array[i] {
            i -= 1;
        }

        if i == 0 {
            return None;
        }

        // find successor to pivot
        let mut j = self.array.len() - 1;
        while self.array[j] <= self.array[i - 1] {
            j -= 1;
        }

        // swap and reverse suffix
        self.array.swap(i - 1, j);
        self.array[i..].reverse();
        Some(self.array.to_owned())
    }
}

// --- //

pub struct PalindromeArray {
    pub array: Vec<isize>
}

pub fn palindrome_array() -> PalindromeArray {
    PalindromeArray { array: to_digits(0) }
}

impl Iterator for PalindromeArray {
    type Item = Vec<isize>;

    fn next(&mut self) -> Option<Self::Item> {
        if !palindrome_increase(&mut self.array) && !palindrome_rotate(&mut self.array) {
            palindrome_expand(&mut self.array);
        }
        Some(self.array.to_owned())
    }
}

fn palindrome_increase(array: &mut Vec<isize>) -> bool {
    let middle = array.len() / 2;
    if array[middle] >= 9 {
        return false;
    }
    if array.len() % 2 == 0 {
        array[middle - 1] += 1;
    }
    array[middle] += 1;
    return true;
}

fn palindrome_rotate(array: &mut Vec<isize>) -> bool {
    let size = array.len();
    for i in size / 2 + 1..size {
        array[i - 1] = 0;
        array[size - i] = 0;

        if array[i] != 9 {
            array[i] += 1;
            array[size - i - 1] += 1;
            return true;
        }
    }
    return false;
}

fn palindrome_expand(array: &mut Vec<isize>) {
    let size = array.len();
    array.clear();
    array.resize(size + 1, 0);
    array[0] = 1;
    array[size] = 1;
}
