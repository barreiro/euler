// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::cmp::min;

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
            l += partition_memoize(remaining - c, *c, sum + c, constrains, cache);
        } else {
            break // assuming constraints are ordered
        }
    }
    cache[remaining as usize][total as usize] = l;
    l
}