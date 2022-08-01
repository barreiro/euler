// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::iter::FromIterator;

#[derive(Default)]
pub struct BitSet {
    bits: Vec<usize>,
    step: usize,
}

impl BitSet {
    pub fn new() -> BitSet {
        BitSet { bits: vec![0, 0, 0, 0], step: usize::BITS as usize }
    }

    /// adds a value to the set. returns 'true' the set did not have this value present
    pub fn insert(&mut self, n: isize) -> bool {
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
    pub fn remove(&mut self, n: isize) -> bool {
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
    pub fn contains(&self, n: isize) -> bool {
        let (index, position) = self.locate(n);
        index < self.bits.len() && self.bits[index] & (1 << position) != 0
    }

    /// number of elements in the set
    pub fn len(&self) -> usize {
        self.bits.iter().map(|b| b.count_ones() as usize).sum()
    }

    /// verifies if the set does not contain any element
    pub fn is_empty(&self) -> bool {
        self.bits.iter().all(|&b| b == 0)
    }

    fn locate(&self, n: isize) -> (usize, usize) {
        if n < i32::MAX as _ {
            ((n as i32 / self.step as i32) as _, (n as i32 % self.step as i32) as _)
        } else {
            (n as usize / self.step, n as usize % self.step)
        }
    }
}

impl FromIterator<isize> for BitSet {
    fn from_iter<I: IntoIterator<Item=isize>>(iter: I) -> BitSet {
        let mut bit_set = BitSet::new();
        iter.into_iter().for_each(|i| { bit_set.insert(i); });
        bit_set
    }
}

impl<'a> FromIterator<&'a isize> for BitSet {
    fn from_iter<I: IntoIterator<Item=&'a isize>>(iter: I) -> BitSet {
        let mut bit_set = BitSet::new();
        iter.into_iter().for_each(|&i| { bit_set.insert(i); });
        bit_set
    }
}
