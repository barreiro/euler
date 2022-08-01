// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::Solver;

// If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.
// If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be used?
// NOTE: Do not count spaces or hyphens.


// number of letters for zero (0), one, two [...] nineteen
const LOOKUP_ONES: &[isize] = &[0, 3, 3, 5, 4, 4, 3, 5, 5, 4, 3, 6, 6, 8, 8, 7, 7, 9, 8, 8];

// number of letters for zero (0), ten, twenty [...] ninety
const LOOKUP_TENS: &[isize] = &[0, 3, 6, 6, 5, 5, 5, 7, 6, 6];

// number of letters for 'and' 'thousand and' and 'hundred and'
const AND_LEN: isize = 3;
const THOUSAND_AND_LEN: isize = 8 + AND_LEN;
const HUNDRED_AND_LEN: isize = 7 + AND_LEN;

pub struct Solver017 {
    pub n: isize,
}

impl Default for Solver017 {
    fn default() -> Self {
        Solver017 { n: 1000 }
    }
}

impl Solver for Solver017 {
    fn solve(&self) -> isize {
        let letter_count = |i| {
            // the number of letters of the thousands, then the hundreds, and finally lookupOnes tens and ones
            let (thousand, hundred, ten, one) = (i / 1000, i % 1000 / 100, i % 100 / 10, i % 10);
            let mut sum = 0;

            if thousand > 0 {
                sum += LOOKUP_ONES[thousand] + THOUSAND_AND_LEN;
            }
            if hundred > 0 {
                sum += LOOKUP_ONES[hundred] + HUNDRED_AND_LEN;
            }

            if ten == 0 && one == 0 {
                sum - AND_LEN
            } else if ten > 1 {
                sum + LOOKUP_TENS[ten] + LOOKUP_ONES[one]
            } else {
                sum + LOOKUP_ONES[i % 100]
            }
        };

        (1..=self.n as usize).map(letter_count).sum()
    }
}
