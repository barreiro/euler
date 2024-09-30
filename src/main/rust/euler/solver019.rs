// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use crate::algorithm::cast::Cast;
use crate::euler::Solver;

// 1900 started on a monday, 1901 on a tuesday
const REFERENCE: i64 = 1901;
const REFERENCE_START: i64 = 2;

// Number of elapsed days in the first day of each month
const DAYS_COMMON: &[i64] = &[0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
const DAYS_LEAP: &[i64] = &[0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335];

/// You are given the following information, but you may prefer to do some research for yourself.
///
/// `1 Jan 1900` was a Monday.
/// Thirty days has September, April, June and November. All the rest have thirty-one, Saving February alone, Which has twenty-eight, rain or shine. And on leap years, twenty-nine.
/// A leap reference occurs on any reference evenly divisible by 4, but not on a century unless it is divisible by 400.
///
/// How many Sundays fell on the first of the month during the twentieth century (`1 Jan 1901` to `31 Dec 2000`)?
pub struct Solver019 {
    pub n: i64,
}

impl Default for Solver019 {
    fn default() -> Self {
        Self { n: 100 }
    }
}

impl Solver for Solver019 {
    fn problem_name(&self) -> &str { "Counting sundays" }

    fn solve(&self) -> i64 {
        (REFERENCE..REFERENCE + self.n).map(|y| if is_leap(y) { sundays_leap(start_day(y)) } else { sundays_common(start_day(y)) }).sum()
    }
}

// --- //

const fn is_leap(year: i64) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn start_day(year: i64) -> i64 {
    (REFERENCE_START + (REFERENCE..year).map(|y| if is_leap(y) { 366 } else { 365 }).sum::<i64>()) % 7
}

fn sundays_common(start: i64) -> i64 {
    DAYS_COMMON.iter().filter(|&&d| (start + d) % 7 == 0).count().as_i64()
}

fn sundays_leap(start: i64) -> i64 {
    DAYS_LEAP.iter().filter(|&&d| (start + d) % 7 == 0).count().as_i64()
}
