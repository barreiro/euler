// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::algorithm::long::int_sqrt;
use euler::algorithm::long::is_palindrome;
use euler::algorithm::long::power_modulo;
use euler::algorithm::prime::miller_rabin;
use euler::algorithm::prime::prime_factors;
use euler::algorithm::combinatorics::partition;

#[test]
fn int_sqrt_test() {
    assert_eq!(int_sqrt(4), 2);
    assert_eq!(int_sqrt(10), 3);
    assert_eq!(int_sqrt(10000), 100);
    assert_eq!(int_sqrt(1787568), 1337);
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
    let natural = vec![ 1, 1, 2, 3, 5, 7, 11, 15, 22, 30, 42, 56, 77, 101, 135, 176, 231, 297, 385, 490, 627, 792, 1002];

    for i in 1..natural.len() {
        let mut constrains = vec![0; i];
        for c in 0..i {
            constrains[c] = 1 + c as isize;
        }
        assert_eq!(natural[i], partition(i as isize, &constrains));
    }
}
