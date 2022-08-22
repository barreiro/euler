// COPYRIGHT (C) 2012 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::convert::TryFrom;
use algorithm::long::DEFAULT_RADIX;

pub trait Cast {
    fn isize(self) -> isize;
    fn usize(self) -> usize;
}

/// safe and easy cast from char
impl Cast for char {
    fn isize(self) -> isize {
        isize::try_from(self.to_digit(u32::try_from(DEFAULT_RADIX).unwrap()).unwrap()).unwrap()
    }
    fn usize(self) -> usize {
        usize::try_from(self.to_digit(u32::try_from(DEFAULT_RADIX).unwrap()).unwrap()).unwrap()
    }
}
// 
// /// safe and easy cast from usize
// impl Cast for isize {
//     fn isize(self) -> isize {
//         self
//     }
//
//     fn usize(self) -> usize {
//         usize::try_from(self).unwrap()
//     }
// }
//
// /// safe and easy cast from usize
// impl Cast for usize {
//     fn isize(self) -> isize {
//         isize::try_from(self).unwrap()
//     }
//     fn usize(self) -> usize {
//         self
//     }
// }
