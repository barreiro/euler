// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

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
    choose_memoize(total as _, min_elements as _, vec![vec![0; min_elements as usize + 1]; total as usize + 1].as_mut())
}

pub fn choose_memoize(total: usize, elements: usize, cache: &mut Vec<Vec<isize>>) -> isize {
    let min_elements = elements.min(total - elements);
    if min_elements == 1 {
        return total as _;
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
    let mut cache = vec![vec![0; 1 + value as usize]; 1 + value as usize];

    partition_memoize(value, value, 0, constrains, &mut cache)
}

fn partition_memoize(remaining: isize, total: isize, sum: isize, constrains: &[isize], cache: &mut Vec<Vec<isize>>) -> isize {
    if remaining == 0 {
        return 1;
    }
    if cache[remaining as usize][total as usize] != 0 {
        return cache[remaining as usize][total as usize];
    }
    let (mut l, bound) = (0, remaining.min(total));
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

pub struct Permutations<F> where F: Fn(&[isize]) -> Option<isize> {
    digits: Vec<isize>,
    predicate: F,
}

pub fn permutations_of<F>(digits: Vec<isize>, predicate: F) -> Permutations<F> where F: Fn(&[isize]) -> Option<isize> {
    Permutations { digits, predicate }
}

pub fn permutations_with<F>(start: isize, size: isize, predicate: F) -> Permutations<F> where F: Fn(&[isize]) -> Option<isize> {
    Permutations { digits: (start..=size).collect::<Vec<_>>(), predicate }
}

impl<F> Iterator for Permutations<F> where F: Fn(&[isize]) -> Option<isize> {
    type Item = isize;

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

impl<F> Permutations<F> where F: Fn(&[isize]) -> Option<isize> {
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
        return true;
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
        return false;
    }

    fn palindrome_expand(&mut self) {
        let size = self.digits.len();
        self.digits = vec![0; size + 1];
        self.digits[0] = 1;
        self.digits[size] = 1;
    }
}
