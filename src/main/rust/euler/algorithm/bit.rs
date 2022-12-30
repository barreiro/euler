// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::iter::FromIterator;

#[derive(Default)]
#[must_use]
pub struct BitSet {
    bits: Vec<u64>,
    step: u32,
}

impl BitSet {
    pub fn new() -> Self {
        Self { bits: vec![0, 0, 0, 0], step: usize::BITS }
    }

    /// adds a value to the set. returns 'true' the set did not have this value present
    pub fn insert(&mut self, n: u64) -> bool {
        let (index, position) = self.locate(n);
        if index >= self.bits.len() {
            // resize aggressively to twice the current size
            self.bits.resize(index << 1, 0);
        }
        let previous = self.bits[index] & (1 << position) == 0;
        self.bits[index] |= 1 << position;
        previous
    }

    /// removes a value from the set. return 'true' if the set did have this value present
    pub fn remove(&mut self, n: u64) -> bool {
        let (index, position) = self.locate(n);
        if index < self.bits.len() {
            let previous = self.bits[index] & (1 << position) == 0;
            self.bits[index] &= !(1 << position);
            previous
        } else {
            false
        }
    }

    /// verifies if a value is present in the set
    #[must_use]
    pub fn contains(&self, n: u64) -> bool {
        let (index, position) = self.locate(n);
        index < self.bits.len() && self.bits[index] & (1 << position) != 0
    }

    /// number of elements in the set
    #[must_use]
    pub fn len(&self) -> usize {
        self.bits.iter().map(|b| b.count_ones() as usize).sum()
    }

    /// verifies if the set does not contain any element
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bits.iter().all(|&b| b == 0)
    }

    /// removes all elements in the set
    pub fn clear(&mut self) {
        self.bits.fill(0);
    }

    #[allow(clippy::cast_possible_truncation)]
    const fn locate(&self, n: u64) -> (usize, u64) {
        if n <= u32::MAX as u64 {
            ((n as u32 / self.step) as usize, (n as u32 % self.step) as u64)
        } else {
            (n as usize / self.step as usize, n % self.step as u64)
        }
    }
}

impl FromIterator<u64> for BitSet {
    fn from_iter<I: IntoIterator<Item=u64>>(iter: I) -> Self {
        let mut bit_set = Self::new();
        iter.into_iter().for_each(|i| { bit_set.insert(i); });
        bit_set
    }
}

impl<'a> FromIterator<&'a u64> for BitSet {
    fn from_iter<I: IntoIterator<Item=&'a u64>>(iter: I) -> Self {
        let mut bit_set = Self::new();
        iter.into_iter().for_each(|&i| { bit_set.insert(i); });
        bit_set
    }
}

// --- //

impl<'a> IntoIterator for &'a BitSet {
    type Item = u64;
    type IntoIter = BitSetIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BitSetIterator { set: self, index: 0, seen: 0 }
    }
}

pub struct BitSetIterator<'a> {
    set: &'a BitSet,
    index: u64,
    seen: usize,
}

impl<'a> Iterator for BitSetIterator<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        (self.seen < self.set.len()).then(|| {
            while !self.set.contains(self.index) {
                self.index += 1;
            }
            self.seen += 1;
            self.index += 1;
            self.index - 1
        })
    }
}
