// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::combinatorics::{partition, partition_with_constrains};
use euler::algorithm::long::{floor_sqrt, gcd, int_sqrt, exact_root};
use euler::algorithm::long::is_palindrome;
use euler::algorithm::long::power_modulo;
use euler::algorithm::prime::miller_rabin;
use euler::algorithm::prime::prime_factors;
use algorithm::combinatorics::permutations_with;
use algorithm::long::factorial;

#[test]
fn gcd_test() {
    assert_eq!(gcd(48, 18), 6);
    assert_eq!(gcd(42, 56), 14);
    assert_eq!(gcd(98, 56), 14);
    assert_eq!(gcd(285, 741), 57);
    assert_eq!(gcd(461952, 116298), 18);
    assert_eq!(gcd(7966496, 314080416), 32);
    assert_eq!(gcd(24826148, 45296490), 526);
}

#[test]
fn int_sqrt_test() {
    assert_eq!(int_sqrt(1), 1);
    assert_eq!(int_sqrt(2), 1);
    assert_eq!(int_sqrt(3), 2);
    assert_eq!(int_sqrt(4), 2);
    assert_eq!(int_sqrt(5), 2);
    assert_eq!(int_sqrt(10), 3);
    assert_eq!(int_sqrt(10000), 100);
    assert_eq!(int_sqrt(10001), 100);
    assert_eq!(int_sqrt(1787568), 1337);
}

#[test]
fn floor_sqrt_test() {
    assert_eq!(floor_sqrt(1), 1);
    assert_eq!(floor_sqrt(2), 1);
    assert_eq!(floor_sqrt(3), 1);
    assert_eq!(floor_sqrt(4), 2);
    assert_eq!(floor_sqrt(5), 2);
    assert_eq!(floor_sqrt(24), 4);
    assert_eq!(floor_sqrt(25), 5);
    assert_eq!(floor_sqrt(26), 5);
    assert_eq!(floor_sqrt(1787568), 1336);
}

#[test]
fn cube_root_test() {
    assert_eq!(exact_root(10, 3), (2, 2));
    assert_eq!(exact_root(27, 3), (3, 0));
    assert_eq!(exact_root(28, 3), (3, 1));
    assert_eq!(exact_root(29, 3), (3, 2));
    assert_eq!(exact_root(30, 3), (3, 3));
    assert_eq!(exact_root(262144, 3), (64, 0));
    assert_eq!(exact_root(1000000, 3), (100, 0));
    assert_eq!(exact_root(100000000, 3), (464, 102656));
}

#[test]
fn palindrome_test() {
    assert_eq!(false, is_palindrome(15));
    assert_eq!(true, is_palindrome(88));
    assert_eq!(false, is_palindrome(15846));
    assert_eq!(true, is_palindrome(84048));
}

#[test]
fn modular_exponentiation_test() {
    assert_eq!(445, power_modulo(4, 13, 497));

    // small values
    assert_eq!(1, power_modulo(3, 0, 7));
    assert_eq!(3, power_modulo(3, 1, 7));
    assert_eq!(2, power_modulo(3, 2, 7));
    assert_eq!(6, power_modulo(3, 3, 7));
    assert_eq!(4, power_modulo(3, 4, 7));
    assert_eq!(5, power_modulo(3, 5, 7));
    assert_eq!(1, power_modulo(3, 6, 7));
}

// --- prime.rs

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
    assert_eq!(true, miller_rabin(2) && miller_rabin(3) && miller_rabin(5) && miller_rabin(7) && miller_rabin(11) && miller_rabin(13));
    assert_eq!(false, miller_rabin(4) || miller_rabin(6) || miller_rabin(8) || miller_rabin(9) || miller_rabin(10) || miller_rabin(12));
}

#[test]
fn miller_rabin_carmichael_test() {
    assert_eq!(false, miller_rabin(561) || miller_rabin(1105) || miller_rabin(1729) || miller_rabin(2465) || miller_rabin(2821) || miller_rabin(6601));
    assert_eq!(false, miller_rabin(101101) || miller_rabin(252602) || miller_rabin(314821) || miller_rabin(340561) || miller_rabin(410041) || miller_rabin(512461));
}

#[test]
fn miller_rabin_long_test() {
    assert_eq!(false, miller_rabin(154639673381) || miller_rabin(585226005592931977) || miller_rabin(7999252175582851) || miller_rabin(55245642489451));
}

// combinatorics.rs

#[test]
fn partition_test() {
    let natural = vec![1, 1, 2, 3, 5, 7, 11, 15, 22, 30, 42, 56, 77, 101, 135, 176, 231, 297, 385, 490, 627, 792, 1002, 1255, 1575, 1958, 2436, 3010, 3718, 4565, 5604, 6842, 8349, 10143, 12310, 14883, 17977, 21637, 26015, 31185, 37338, 44583, 53174, 63261, 75175, 89134, 105558, 124754, 147273, 173525];
    natural.iter().enumerate().for_each(|(n, &p)| assert_eq!(partition(n as _), p));
    natural.iter().enumerate().for_each(|(n, &p)| assert_eq!(partition_with_constrains(n as _, &(1..=n as _).collect::<Vec<_>>()), p));
}

#[test]
fn permutation_test() {
    assert_eq!(factorial(5 + 1) as usize, permutations_with(0, 5, |p| Some(p.to_vec())).count());
    
    let mut perm = permutations_with(0, 1, |p| Some(p.to_vec()));
    assert_eq!(Some(vec![0, 1]), perm.next());
    assert_eq!(Some(vec![1, 0]), perm.next());
    assert_eq!(None, perm.next());

}