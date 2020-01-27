// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::iter::FromIterator;
use std::mem::size_of;

pub struct BitSet {
    bits: Vec<usize>,
    step: usize,
}

impl BitSet {
    pub fn new() -> BitSet {
        BitSet { bits: vec![0, 0, 0, 0], step: 8 * size_of::<usize>() }
    }

    /// Adds a value to the set. If the set did not have this value present, `true` is returned.
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

    pub fn contains(&self, n: isize) -> bool {
        let (index, position) = self.locate(n);
        index < self.bits.len() && self.bits[index] & (1 << position) != 0
    }

    pub fn len(&self) -> usize {
        self.bits.iter().map(|b| b.count_ones() as usize).sum()
    }

    fn locate(&self, n: isize) -> (usize, usize) {
        if n < i32::max_value() as _ {
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
