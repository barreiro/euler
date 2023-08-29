// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::str::FromStr;

use algorithm::cast::Cast;
use algorithm::digits::digits_iter;
use algorithm::io::load_default_data;
use Solver;

const MINIMAL_FORM: &[usize] = &[0, 1, 2, 3, 2, 1, 2, 3, 4, 2];

/// For a number written in Roman numerals to be considered valid there are basic rules which must be followed.
/// Even though the rules allow some numbers to be expressed in more than one way there is always a "best" way of writing a particular number.
///
/// For example, it would appear that there are at least six ways of writing the number sixteen:
/// ```
/// IIIIIIIIIIIIIIII
/// VIIIIIIIIIII
/// VVIIIIII
/// XIIIIII
/// VVVI
/// XVI
/// ```
/// However, according to the rules only `XIIIIII` and `XVI` are valid, and the last example is considered to be the most efficient, as it uses the least number of numerals.
///
/// The 11K text file, `roman.txt` (right click and 'Save Link/Target As...'), contains one thousand numbers written in valid, but not necessarily minimal, Roman numerals; see About... Roman Numerals for the definitive rules for this problem.
/// Find the number of characters saved by writing each of these in their minimal form.
///
/// Note: You can assume that all the Roman numerals in the file contain no more than four consecutive identical units.
pub struct Solver089 {
    pub n: usize,
    pub input: String,
}

impl Default for Solver089 {
    fn default() -> Self {
        Self { n: 1_000, input: load_default_data(89) }
    }
}

impl Solver for Solver089 {
    fn problem_name(&self) -> &str { "Roman numerals" }

    fn solve(&self) -> i64 {
        self.input.lines().take(self.n).map(|s| (s.len() - s.parse::<Roman>().expect("Input string should contain roman numeral").minimal_len()).as_i64()).sum()
    }
}

// --- //

struct Roman {
    digits: Vec<u64>,
}

impl FromStr for Roman {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            digits: s.chars().filter_map(|c| match c {
                'I' => Some(1),
                'V' => Some(5),
                'X' => Some(10),
                'L' => Some(50),
                'C' => Some(100),
                'D' => Some(500),
                'M' => Some(1000),
                _ => None
            }).collect()
        })
    }
}

impl Roman {
    fn minimal_len(&self) -> usize {
        // conversion from roman to decimal. sum all digits and then subtract twice the 'out of order'
        let value = self.digits.iter().scan(u64::MAX, |previous, &d| {
            let v = if d > *previous { d - 2 * *previous } else { d };
            *previous = d;
            Some(v)
        }).sum::<u64>();

        value.as_usize() / 1000 + digits_iter(value % 1000).map(|d| MINIMAL_FORM[d.as_usize()]).sum::<usize>()
    }
}
