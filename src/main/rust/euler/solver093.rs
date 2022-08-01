// COPYRIGHT (C) 2022 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::ops::{Add, Div, Mul, Sub};

use euler::algorithm::combinatorics::{combinations_with, permutations_of_set_with};
use euler::algorithm::long::from_digits;
use euler::Solver;

// By using each of the digits from the set, {1, 2, 3, 4}, exactly once, and making use of the four arithmetic operations (+, −, *, /) and brackets/parentheses, it is possible to form different positive integer targets.
// For example:
//       8 = (4 * (1 + 3)) / 2
//      14 = 4 * (3 + 1 / 2)
//      19 = 4 * (2 + 3) − 1
//      36 = 3 * 4 * (2 + 1)
//
// Note that concatenations of the digits, like 12 + 34, are not allowed.
//
// Using the set, {1, 2, 3, 4}, it is possible to obtain thirty-one different target numbers of which 36 is the maximum, and each of the numbers 1 to 28 can be obtained before encountering the first non-expressible number.
// Find the set of four distinct digits, a < b < c < d, for which the longest set of consecutive positive integers, 1 to n, can be obtained, giving your answer as a string: abcd.

#[derive(Clone, Copy, PartialEq)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

const _OPERATIONS: &[Operation] = &[Operation::Add, Operation::Sub, Operation::Mul, Operation::Div];

pub struct Solver093 {
    pub n: isize,
}

impl Default for Solver093 {
    fn default() -> Self {
        Solver093 { n: 4 }
    }
}

impl Solver for Solver093 {
    fn solve(&self) -> isize {
        // let _ops = permutations_with_repetition_of_set(_OPERATIONS.to_vec(), self.n as usize - 1).collect::<Vec<_>>();
        let _ops = [_OPERATIONS.iter().map(|o| o.to_owned()).collect::<Vec<_>>()];
        let _consecutive_rpn = |set: &Vec<isize>| {
            let mut digits = set.to_vec();
            digits.reverse(); // need to reverse for permutations to work
            let mut result = permutations_of_set_with(digits, |set| Some(_recursive_rpn(set, &_ops))).reduce(|a, b| [a, b].concat()).unwrap_or_default();
            result.sort_unstable();
            result.dedup();
            result.iter().enumerate().take_while(|(n, &e)| n + 1 == e as usize).count()
        };

        // combinations_with((1..=9).collect::<Vec<_>>(), self.n as _, |c| Some(c.to_vec())).max_by_key(_consecutive_rpn).map(from_digits).unwrap();

        let consecutive_expansion = |set: &Vec<isize>| expansion(set).iter().enumerate().take_while(|(n, &e)| n + 1 == e as usize).count();
        combinations_with((1..=9).collect::<Vec<_>>(), self.n as _, |c| Some(c.to_vec())).max_by_key(consecutive_expansion).map(from_digits).unwrap()
    }
}

fn _recursive_rpn(set: &[isize], ops: &[Vec<Operation>]) -> Vec<isize> {
    let mut results = vec![];
    ops.iter().for_each(|op| _recursive_rpn_op(&mut vec![], &mut set.to_vec(), &mut op.to_vec(), &mut results));
    results
}

fn _recursive_rpn_op(stack: &mut Vec<Rational>, set: &mut Vec<isize>, ops: &mut Vec<Operation>, results: &mut Vec<isize>) {
    if stack.len() > 1 && !ops.is_empty() {
        let (a, b) = (stack.pop().unwrap(), stack.pop().unwrap());
        let op = ops.pop().unwrap();
        stack.push(match op {
            Operation::Add => { a + b }
            Operation::Sub => { a - b }
            Operation::Mul => { a * b }
            Operation::Div => { a / b }
        });
        _recursive_rpn_op(stack, set, ops, results);
        ops.push(op);
        stack.pop();
        stack.push(b);
        stack.push(a);
    }
    if !set.is_empty() {
        let s = set.pop().unwrap();
        stack.push(Rational::from(&s));
        _recursive_rpn_op(stack, set, ops, results);
        stack.pop();
        set.push(s);
    }
    if set.is_empty() && ops.is_empty() {
        if let Some(result) = stack.get(0).unwrap().as_isize() {
            if result > 0 {
                results.push(result)
            }
        }
    }
}

// converts from a set of integers into all the possible target results
fn expansion(set: &[isize]) -> Vec<isize> {
    let rationals = set.iter().map(Rational::from).collect::<Vec<_>>();
    let mut result = rational_expansion(&rationals).iter().filter_map(|r| r.as_isize()).filter(|&x| x > 0).collect::<Vec<_>>();
    result.sort_unstable();
    result.dedup();
    result
}

fn rational_expansion(set: &[Rational]) -> Vec<Rational> {
    if set.len() == 1 {
        set.to_vec()
    } else if set.len() == 2 {
        rational_product(&[set[0]], &[set[1]])
    } else {
        // split into all possible partitions (by increasing the size len) and recurse
        let mut result = vec![];
        for len in 1..=set.len() >> 1 {
            combinations_with(set.to_vec(), len, |comb| {
                let mut working_set = set.to_vec();
                working_set.retain(|w| !comb.contains(w));
                Some((comb.to_vec(), working_set))
            }).for_each(|(p1, p2)| result.append(&mut rational_product(&rational_expansion(&p1), &rational_expansion(&p2))));
            // memoization in the expansion showed no significant advantage
        }
        result
    }
}

// cartesian product of two sets of rationals (over all the possible operations - note the symmetry)
fn rational_product(a: &[Rational], b: &[Rational]) -> Vec<Rational> {
    let mut result = Vec::with_capacity(a.len() * b.len() * 6);
    for &a in a {
        for &b in b {
            result.append(&mut vec![a + b, a * b, a - b, a / b, b - a, b / a]);
        }
    }
    result
}

// --- //

#[derive(Clone, Copy, PartialEq)]
struct Rational {
    n: isize,
    d: isize,
}

// impl Debug for Rational {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         if self.d == 1 { write!(f, "{}", self.n) } else { write!(f, "{:?}/{:?}", self.n, self.d) }
//     }
// }

impl From<&isize> for Rational {
    fn from(&n: &isize) -> Self {
        Rational { n, d: 1 }
    }
}

impl Add for Rational {
    type Output = Rational;

    fn add(self, rhs: Self) -> Self::Output {
        Rational { n: self.n * rhs.d + rhs.n * self.d, d: self.d * rhs.d }
    }
}

impl Sub for Rational {
    type Output = Rational;

    fn sub(self, rhs: Self) -> Self::Output {
        Rational { n: self.n * rhs.d - rhs.n * self.d, d: self.d * rhs.d }
    }
}

impl Mul for Rational {
    type Output = Rational;

    fn mul(self, rhs: Self) -> Self::Output {
        Rational { n: self.n * rhs.n, d: self.d * rhs.d }
    }
}

impl Div for Rational {
    type Output = Rational;

    fn div(self, rhs: Self) -> Self::Output {
        Rational { n: self.n * rhs.d, d: self.d * rhs.n }
    }
}

impl Rational {
    const fn as_isize(&self) -> Option<isize> {
        if self.d == 1 {
            Some(self.n)
        } else if self.d != 0 && self.n % self.d == 0 {
            Some(self.n / self.d)
        } else {
            None
        }
    }
}
