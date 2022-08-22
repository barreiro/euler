// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::factor::sum_of_factors;
use euler::algorithm::long::{ceil_sqrt, gcd, is_even, pentagonal, square};

/// method for calculation the combinations of a certain number of elements in a total number of places.
/// uses iteration instead of the formula with factorials.
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

/// calculates the number of integer partitions of a value, given a set of (ordered) constrains.
pub fn partition_with_constrains(value: isize, constrains: &[isize]) -> isize {
    partition_with_constrains_memoize(value as _, value as _, 0, constrains, &mut vec![vec![0; 1 + value as usize]; 1 + value as usize])
}

fn partition_with_constrains_memoize(total: usize, remaining: usize, _level: usize, constrains: &[isize], cache: &mut Vec<Vec<isize>>) -> isize {
    if remaining == 0 || remaining == constrains[0] as _ {
        1
    } else if remaining < constrains[0] as _ {
        0
    } else {
        if cache[total][remaining] == 0 {
            let ceil = remaining.min(total);
            cache[total][remaining] = constrains.iter().map(|&c| c as _).take_while(|&c| c <= ceil).map(|c| partition_with_constrains_memoize(c, remaining - c, _level + 1, constrains, cache)).sum();
        }
        cache[total][remaining]
    }
}

/// calculates the number of integer partitions of a value
pub fn partition(value: isize) -> isize {
    let (mut p, sum_of_factors) = (vec![0; 1 + value as usize], (0..=value).map(|v| v + sum_of_factors(v)).collect::<Vec<_>>());
    (0..=1.min(value as _)).for_each(|v| p[v] = 1);
    (2..=value as _).for_each(|v| p[v] = (0..v).map(|k| sum_of_factors[v - k] * p[k]).sum::<isize>() / v as isize);
    p[value as usize]
}

/// finds the least number with a certain integer partition value, with a certain modulo
pub fn partition_modulo_find(modulo: isize, predicate: isize) -> isize {
    let mut cache = Vec::with_capacity(ceil_sqrt(modulo) as _);
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

/// provides an iterator of the permutations of the given elements that satisfy a given mapping predicate. requires the elements to be sorted in order to provide all possible permutations
pub fn permutations_of_set_with<T, F, R>(elements: Vec<T>, predicate: F) -> impl Iterator<Item=R> where T: PartialOrd, F: Fn(&[T]) -> Option<R> {
    Permutations { elements, predicate }
}

/// provides an iterator of permutations of the numbers between start and size (inclusive) that satisfy a given mapping predicate.
pub fn permutations_with<F, R>(start: isize, size: isize, predicate: F) -> impl Iterator<Item=R> where F: Fn(&[isize]) -> Option<R> {
    permutations_of_set_with((start..=size).collect::<Vec<_>>(), predicate)
}

struct Permutations<T, F, R> where T: PartialOrd, F: Fn(&[T]) -> Option<R> {
    elements: Vec<T>,
    predicate: F,
}

impl<T, F, R> Iterator for Permutations<T, F, R> where T: PartialOrd, F: Fn(&[T]) -> Option<R> {
    type Item = R;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.elements.is_empty() {
                return None;
            }
            let result = (self.predicate)(&self.elements);
            if !self.permutate() {
                self.elements.clear();
            }
            if result.is_some() {
                return result;
            }
        }
    }
}

impl<T, F, R> Permutations<T, F, R> where T:PartialOrd, F: Fn(&[T]) -> Option<R> {
    pub fn permutate(&mut self) -> bool {
        // find non-increasing suffix
        let mut i = self.elements.len() - 1;
        while i > 0 && self.elements[i - 1] >= self.elements[i] {
            i -= 1;
        }

        if i == 0 {
            return false;
        }

        // find successor to pivot
        let mut j = self.elements.len() - 1;
        while self.elements[j] <= self.elements[i - 1] {
            j -= 1;
        }

        // swap and reverse suffix
        self.elements.swap(i - 1, j);
        self.elements[i..].reverse();
        true
    }
}

// --- //

/// provides an iterator of combinations of the elements (with repetition of elements)
pub fn permutations_with_repetition_of_set<T>(elements: Vec<T>, size: usize) -> impl Iterator<Item=Vec<T>> where T: Copy {
    PermutationsWithRepetition { elements, indexes: vec![0; size] }
}

struct PermutationsWithRepetition<T> where T: Copy {
    elements: Vec<T>,
    indexes: Vec<usize>,
}

impl<T> Iterator for PermutationsWithRepetition<T> where T: Copy {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.indexes[0] >= self.elements.len() {
            None
        } else {
            let mut result = Vec::with_capacity(self.indexes.len());
            self.indexes.iter().for_each(|&i| result.push(self.elements[i]));
            for i in (0..self.indexes.len()).rev() {
                self.indexes[i] += 1;
                if i != 0 && self.indexes[i] >= self.elements.len() {
                    self.indexes[i] = 0;
                } else {
                    break;
                }
            }
            Some(result)
        }
    }
}

// --- //

/// provides an iterator of combinations of elements that satisfy a given mapping predicate
pub fn combinations_with<T, F, R>(elements: Vec<T>, size: usize, predicate: F) -> impl Iterator<Item=R> where T: Copy, F: Fn(&[T]) -> Option<R> {
    let mut pattern = (0..size as isize).rev().collect::<Vec<_>>();
    pattern[0] -= 1;
    Combinations { elements, pattern, predicate }
}

// this implementation uses a vec of positions. it can be improved for elements.len() < isize::BITS using a trick known as Gospher's Hack
struct Combinations<T, F, R> where F: Fn(&[T]) -> Option<R> {
    // use an isize pattern to allow combinations of size 1
    elements: Vec<T>,
    pattern: Vec<isize>,
    predicate: F,
}

impl<T, F, R> Iterator for Combinations<T, F, R> where F: Fn(&[T]) -> Option<R>, T: Copy {
    type Item = R;

    fn next(&mut self) -> Option<Self::Item> {
        while self.pattern_increase() {
            let result = (self.predicate)(self.pattern.iter().map(|&i| self.elements[i as usize]).collect::<Vec<_>>().as_slice());
            if result.is_some() {
                return result;
            }
        }
        None
    }
}

impl<I, F, R> Combinations<I, F, R> where F: Fn(&[I]) -> Option<R> {
    fn pattern_increase(&mut self) -> bool {
        for i in 0..self.pattern.len() {
            if (self.pattern[i] + i as isize) < (self.elements.len() - 1) as isize {
                self.pattern[i] += 1;
                for j in 0..i {
                    self.pattern[j] = self.pattern[i] + (i - j) as isize;
                }
                return true;
            }
        }
        false
    }
}

// --- //

/// iteration over all Pythagorean triples
/// `(a < b < c)` but each can have smaller values later on the iteration
pub fn pythagorean_triplets() -> impl Iterator<Item=(isize, isize, isize)> {
    PythagoreanTriplets { m: 1, n: 0 }
}

/// iteration over all Pythagorean triples
/// `(a < b < c)` but each can have smaller values later on the iteration
pub fn pythagorean_triplets_lower_bound(bound: isize) -> impl Iterator<Item=(isize, isize, isize)> {
    PythagoreanTriplets { m: 1, n: 0 }.take_while(move |&(a, b, c)| c - b != 2 && a < square(bound))
}

struct PythagoreanTriplets {
    m: isize,
    n: isize,
}

impl Iterator for PythagoreanTriplets {
    type Item = (isize, isize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        // A primitive Pythagorean triple additionally requires:
        // m and n have opposite parity – i.e. if one is odd, the other must be even.
        // m and n are coprime – i.e. they have no common integer factors greater than 1.
        while {
            self.n += 2;
            if self.n >= self.m {
                self.m += 1;
                self.n = if is_even(self.m) { 1 } else { 2 };
            }
            gcd(self.m, self.n) != 1
        } {}

        // uses Euclides Formula --- a=m^2-n^2 --- b=2nm --- c=m^2+n^2 --- with m>n
        let (m_square, n_square) = (square(self.m), square(self.n));
        let (a, b, c) = (m_square - n_square, 2 * self.m * self.n, m_square + n_square);
        Some((a.min(b), a.max(b), c))
    }
}

// --- //

/// iterator on palindromes (numbers that read the same in both ways)
pub fn palindromes() -> impl Iterator<Item=Vec<isize>> {
    Palindromes { digits: vec![0] }
}

struct Palindromes {
    pub digits: Vec<isize>,
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
