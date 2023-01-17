// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::combinatorics::partition;
use algorithm::combinatorics::partition_with_constrains;
use algorithm::combinatorics::permutations_of_digits_with;
use algorithm::digits::Digits;
use algorithm::factor::sum_of_factors;
use algorithm::filter::{is_palindrome, is_pandigital};
use algorithm::long::factorial;
use algorithm::long::gcd;
use algorithm::long::pow_mod;
use algorithm::prime::descending_primes;
use algorithm::prime::miller_rabin;
use algorithm::prime::prime_factors;
use algorithm::prime::primes_up_to;
use algorithm::prime::primes_wheel_up_to;
use algorithm::root::exact_root;
use algorithm::root::floor_sqrt;
use algorithm::root::int_sqrt;

#[test]
fn gcd_test() {
    assert_eq!(gcd(30, 12), 6);
    assert_eq!(gcd(48, 18), 6);
    assert_eq!(gcd(42, 56), 14);
    assert_eq!(gcd(98, 56), 14);
    assert_eq!(gcd(285, 741), 57);
    assert_eq!(gcd(461952, 116298), 18);
    assert_eq!(gcd(7966496, 314080416), 32);
    assert_eq!(gcd(24826148, 45296490), 526);
}

#[test]
fn modular_exponentiation_test() {
    assert_eq!((0..10).map(|exp| pow_mod(3, exp, 7)).collect::<Vec<_>>(), [1, 3, 2, 6, 4, 5, 1, 3, 2, 6]);

    assert_eq!(445, pow_mod(4, 13, 497));
}

// --- filter.rs

#[test]
fn palindrome_test() {
    assert!(is_palindrome(&88) && is_palindrome(&84048) && is_palindrome(&38411483) && is_palindrome(&384101483));
    assert!(!is_palindrome(&15) && !is_palindrome(&15846) && !is_palindrome(&9840486) && !is_palindrome(&38413483));
    assert!((1..10000).all(|n| is_palindrome(&n) == Digits::from(n).is_palindrome()));
}

#[test]
fn pandigital_test() {
    assert!(is_pandigital(&123456789) && is_pandigital(&12345) && is_pandigital(&54321) && is_pandigital(&192837465));
    assert!(!is_pandigital(&151) && !is_pandigital(&10) && !is_pandigital(&123451) && !is_pandigital(&76543210));
    assert!((1..10000).all(|n| is_pandigital(&n) == Digits::from(n).is_pandigital()));
}

// --- root.rs

#[test]
fn int_sqrt_test() {
    assert_eq!((0..10).map(int_sqrt).collect::<Vec<_>>(), [0, 1, 1, 2, 2, 2, 2, 3, 3, 3]);
    assert_eq!(int_sqrt(10000), 100);
    assert_eq!(int_sqrt(10001), 100);
    assert_eq!(int_sqrt(1787568), 1337);
}

#[test]
fn floor_sqrt_test() {
    assert_eq!([1, 2, 3, 4, 5, 6, 24, 25, 26].iter().map(|&n| floor_sqrt(n)).collect::<Vec<_>>(), [1, 1, 1, 2, 2, 2, 4, 5, 5]);

    assert_eq!(floor_sqrt(1787568), 1336);
}

#[test]
fn cube_root_test() {
    assert_eq!([10, 27, 28, 29, 30].iter().map(|&n| exact_root(n, 3)).collect::<Vec<_>>(), [(2, 2), (3, 0), (3, 1), (3, 2), (3, 3)]);

    assert_eq!(exact_root(262144, 3), (64, 0));
    assert_eq!(exact_root(1000000, 3), (100, 0));
    assert_eq!(exact_root(100000000, 3), (464, 102656));
}

// --- factor.rs

#[test]
fn sum_of_factors_test() {
    assert_eq!((0..10).map(sum_of_factors).collect::<Vec<_>>(), [0, 0, 1, 1, 3, 1, 6, 1, 7, 4]);

    assert_eq!(sum_of_factors(28), 28);
    assert_eq!(sum_of_factors(220), 284);
    assert_eq!(sum_of_factors(284), 220);
    assert_eq!(sum_of_factors(12496), 14288);
}

// --- prime.rs

#[test]
fn prime_iterator_test() {
    assert_eq!(primes_up_to(10).collect::<Vec<_>>(), [2, 3, 5, 7]);
    assert_eq!(primes_wheel_up_to(10).collect::<Vec<_>>(), [2, 3, 5, 7]);
    assert_eq!(descending_primes(10).collect::<Vec<_>>(), [7, 5, 3, 2]);
}

#[test]
fn prime_factors_test() {
    assert_eq!(prime_factors(4), [(2, 2)].iter().cloned().collect());
    assert_eq!(prime_factors(21), [(3, 1), (7, 1)].iter().cloned().collect());
    assert_eq!(prime_factors(23), [(23, 1)].iter().cloned().collect());
    assert_eq!(prime_factors(840), [(2, 3), (3, 1), (5, 1), (7, 1)].iter().cloned().collect());
    assert_eq!(prime_factors(1031), [(1031, 1)].iter().cloned().collect());
}

#[test]
fn miller_rabin_test() {
    assert!(miller_rabin(2) && miller_rabin(3) && miller_rabin(5) && miller_rabin(7) && miller_rabin(11) && miller_rabin(13));
    assert!(!(miller_rabin(1) || miller_rabin(4) || miller_rabin(6) || miller_rabin(8) || miller_rabin(9) || miller_rabin(10) || miller_rabin(12)));
}

#[test]
fn miller_rabin_carmichael_test() {
    assert!(!(miller_rabin(561) || miller_rabin(1105) || miller_rabin(1729) || miller_rabin(2465) || miller_rabin(2821) || miller_rabin(6601)));
    assert!(!(miller_rabin(101101) || miller_rabin(252602) || miller_rabin(314821) || miller_rabin(340561) || miller_rabin(410041) || miller_rabin(512461)));
}

#[test]
fn miller_rabin_long_test() {
    assert!(!(miller_rabin(154639673381) || miller_rabin(585226005592931977) || miller_rabin(7999252175582851) || miller_rabin(55245642489451)));
}

// combinatorics.rs

#[test]
fn partition_test() {
    let natural = vec![1, 1, 2, 3, 5, 7, 11, 15, 22, 30, 42, 56, 77, 101, 135, 176, 231, 297, 385, 490, 627, 792, 1002, 1255, 1575, 1958, 2436, 3010, 3718, 4565, 5604, 6842, 8349, 10143, 12310, 14883, 17977, 21637, 26015, 31185, 37338, 44583, 53174, 63261, 75175, 89134, 105558, 124754, 147273, 173525];
    natural.iter().enumerate().for_each(|(n, &p)| assert_eq!(partition(n as u64), p));
    natural.iter().enumerate().for_each(|(n, &p)| assert_eq!(partition_with_constrains(n as u64, &(1..=n as u64).collect::<Vec<_>>()), p));
}

#[test]
fn permutation_test() {
    assert_eq!(factorial(5 + 1) as usize, permutations_of_digits_with(0, 5, |p| Some(p.to_vec())).count());

    let mut perm = permutations_of_digits_with(0, 1, |p| Some(p.to_vec()));
    assert_eq!(Some(vec![0, 1]), perm.next());
    assert_eq!(Some(vec![1, 0]), perm.next());
    assert_eq!(None, perm.next());
}
