// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::iter::FromIterator;
use std::mem::size_of;

pub struct BitSet {
    bits: Vec<usize>,
    step: usize,
}

pub fn bit_set() -> BitSet {
    BitSet { bits: vec![], step: 8 * size_of::<usize>() }
}

impl BitSet {

    /// Adds a value to the set. If the set did not have this value present, `true` is returned.
    pub fn insert(&mut self, n: isize) -> bool {
        let (index, position) = (n as usize / self.step, n as usize % self.step);
        if index >= self.bits.len() {
            self.bits.resize(index + 1, 0);
        }
        let previous = self.bits[index] & (1 << position) == 0;
        self.bits[index] |= 1 << position;
        previous
    }

    pub fn contains(&self, n: isize) -> bool {
        let (index, position) = (n as usize / self.step, n as usize % self.step);
        index < self.bits.len() && self.bits[index] & (1 << position) != 0
    }
}

impl FromIterator<isize> for BitSet {
    fn from_iter<I: IntoIterator<Item=isize>>(iter: I) -> BitSet {
        let mut bit_set = bit_set();
        iter.into_iter().for_each(|i| { bit_set.insert(i); });
        bit_set
    }
}

impl<'a> FromIterator<&'a isize> for BitSet {
    fn from_iter<I: IntoIterator<Item=&'a isize>>(iter: I) -> BitSet {
        let mut bit_set = bit_set();
        iter.into_iter().for_each(|&i| { bit_set.insert(i); });
        bit_set
    }
}
