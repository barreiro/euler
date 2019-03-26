// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::Solver;

// You are given the following information, but you may prefer to do some research for yourself.
//
// 1 Jan 1900 was a Monday.
// Thirty days has September, April, June and November. All the rest have thirty-one, Saving February alone, Which has twenty-eight, rain or shine. And on leap years, twenty-nine.
// A leap reference occurs on any reference evenly divisible by 4, but not on a century unless it is divisible by 400.
//
// How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?

// 1900 started on a monday, 1901 on a tuesday
const REFERENCE: isize = 1901;
const REFERENCE_START: isize = 2;

//// Number of elapsed days in the first referenceStart of each month
const DAYS_COMMON: &[isize] = &[0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
const DAYS_LEAP: &[isize] = &[0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335];

pub struct Solver019 {
    pub n: isize,
}

impl Default for Solver019 {
    fn default() -> Self {
        Solver019 {
            n: 100
        }
    }
}

impl Solver for Solver019 {
    fn solve(&self) -> isize {
        let mut sum = 0;
        for y in REFERENCE..REFERENCE + self.n {
            sum += if is_leap(y) { sundays_leap(start_day(y)) } else { sundays_common(start_day(y)) }
        }
        sum
    }
}

// --- //

fn is_leap(year: isize) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn start_day(year: isize) -> isize {
    let mut sum = REFERENCE_START;
    for y in REFERENCE..year {
        sum += if is_leap(y) { 366 } else { 365 }
    }
    sum % 7
}

fn sundays_common(start: isize) -> isize {
    let mut sum = 0;
    for d in DAYS_COMMON {
        sum += if (start + d) % 7 == 0 { 1 } else { 0 }
    }
    sum
}

fn sundays_leap(start: isize) -> isize {
    let mut sum = 0;
    for d in DAYS_LEAP {
        sum += if (start + d) % 7 == 0 { 1 } else { 0 }
    }
    sum
}
