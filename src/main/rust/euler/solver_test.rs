// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use euler::*;

#[test]
fn solver_001_test() {
    assert_eq!(Solver001::default().solve(), 233168);

    assert_eq!(Solver001 { n: 6, ..Default::default() }.solve(), 8);
    assert_eq!(Solver001 { n: 7, ..Default::default() }.solve(), 14);
    assert_eq!(Solver001 { n: 10, ..Default::default() }.solve(), 23);
    assert_eq!(Solver001 { n: 30, ..Default::default() }.solve(), 195);
    assert_eq!(Solver001 { n: 1000000000, ..Default::default() }.solve(), 233333333166666668);
}

#[test]
fn solver_002_test() {
    assert_eq!(Solver002::default().solve(), 4613732);

    assert_eq!(Solver002 { n: 100 }.solve(), 44);
    assert_eq!(Solver002 { n: 1000000000000 }.solve(), 478361013020);
}

#[test]
fn solver_003_test() {
    assert_eq!(Solver003::default().solve(), 6857);

    assert_eq!(Solver003 { n: 12 }.solve(), 3);
    assert_eq!(Solver003 { n: 13195 }.solve(), 29);
    assert_eq!(Solver003 { n: 1000000000031 }.solve(), 85302397);
}

#[test]
fn solver_004_test() {
    assert_eq!(Solver004::default().solve(), 906609);

    assert_eq!(Solver004 { n: 1 }.solve(), 9);
    assert_eq!(Solver004 { n: 2 }.solve(), 9009);
    assert_eq!(Solver004 { n: 4 }.solve(), 99000099);
}

#[test]
fn solver_005_test() {
    let check = |n: isize, solution: isize| -> bool {
        for i in 1..n {
            if solution % i != 0 {
                return false;
            }
        }
        true
    };

    assert_eq!(true, check(6, 60));
    assert_eq!(true, check(10, 2520));
    assert_eq!(true, check(20, 232792560));
    assert_eq!(true, check(40, 5342931457063200));

    assert_eq!(Solver005::default().solve(), 232792560);

    assert_eq!(Solver005 { n: 6 }.solve(), 60);
    assert_eq!(Solver005 { n: 10 }.solve(), 2520);
    assert_eq!(Solver005 { n: 40 }.solve(), 5342931457063200);
}

#[test]
fn solver_006_test() {
    assert_eq!(Solver006::default().solve(), 25164150);

    assert_eq!(Solver006 { n: 5 }.solve(), 170);
    assert_eq!(Solver006 { n: 10 }.solve(), 2640);
    assert_eq!(Solver006 { n: 200 }.solve(), 401323300);
}

#[test]
fn solver_007_test() {
    assert_eq!(Solver007::default().solve(), 104743);

    assert_eq!(Solver007 { n: 1 }.solve(), 2);
    assert_eq!(Solver007 { n: 6 }.solve(), 13);
    assert_eq!(Solver007 { n: 100000 }.solve(), 1299709);
}

#[test]
fn solver_008_test() {
    assert_eq!(Solver008::default().solve(), 23514624000);

    assert_eq!(Solver008 { n: 1, ..Default::default() }.solve(), 9);
    assert_eq!(Solver008 { n: 4, ..Default::default() }.solve(), 5832);
    assert_eq!(Solver008 { n: 20, ..Default::default() }.solve(), 240789749760000);
}

#[test]
fn solver_009_test() {
    assert_eq!(Solver009::default().solve(), 31875000);

    assert_eq!(Solver009 { n: 10 }.solve(), 0);
    assert_eq!(Solver009 { n: 12 }.solve(), 60);
    assert_eq!(Solver009 { n: 20000 }.solve(), 265387500000);
}

#[test]
fn solver_010_test() {
    assert_eq!(Solver010::default().solve(), 142913828922);

    assert_eq!(Solver010 { n: 5 }.solve(), 5);
    assert_eq!(Solver010 { n: 10 }.solve(), 17);
    assert_eq!(Solver010 { n: 1000000 }.solve(), 37550402023);
}

#[test]
fn solver_011_test() {
    assert_eq!(Solver011::default().solve(), 70600674);

    assert_eq!(Solver011 { n: 1, ..Default::default() }.solve(), 99);
    assert_eq!(Solver011 { n: 2, ..Default::default() }.solve(), 9603);
    assert_eq!(Solver011 { n: 3, ..Default::default() }.solve(), 811502);
    assert_eq!(Solver011 { n: 5, ..Default::default() }.solve(), 3318231678);
    assert_eq!(Solver011 { n: 6, ..Default::default() }.solve(), 188210512710);
}

#[test]
fn solver_012_test() {
    assert_eq!(Solver012::default().solve(), 76576500);

    assert_eq!(Solver012 { n: 4 }.solve(), 6);
    assert_eq!(Solver012 { n: 5 }.solve(), 28);
    assert_eq!(Solver012 { n: 10 }.solve(), 120);
    assert_eq!(Solver012 { n: 100 }.solve(), 73920);
}

#[test]
fn solver_013_test() {
    assert_eq!(Solver013::default().solve(), 5537376230);

    assert_eq!(Solver013 { n: 1, ..Default::default() }.solve(), 5);
    assert_eq!(Solver013 { n: 2, ..Default::default() }.solve(), 55);
    assert_eq!(Solver013 { n: 3, ..Default::default() }.solve(), 553);
    assert_eq!(Solver013 { n: 4, ..Default::default() }.solve(), 5537);
    assert_eq!(Solver013 { n: 5, ..Default::default() }.solve(), 55373);
    assert_eq!(Solver013 { n: 6, ..Default::default() }.solve(), 553737);
    assert_eq!(Solver013 { n: 15, ..Default::default() }.solve(), 553737623039087);
}

#[test]
fn solver_014_test() {
    assert_eq!(Solver014::default().solve(), 837799);

    assert_eq!(Solver014 { n: 5 }.solve(), 3);
    assert_eq!(Solver014 { n: 100 }.solve(), 97);
    assert_eq!(Solver014 { n: 5000000 }.solve(), 3732423);
}

#[test]
fn solver_015_test() {
    assert_eq!(Solver015::default().solve(), 137846528820);

    assert_eq!(Solver015 { n: 1 }.solve(), 2);
    assert_eq!(Solver015 { n: 2 }.solve(), 6);
    assert_eq!(Solver015 { n: 3 }.solve(), 20);
    assert_eq!(Solver015 { n: 5 }.solve(), 252);
    assert_eq!(Solver015 { n: 10 }.solve(), 184756);
    assert_eq!(Solver015 { n: 30 }.solve(), 118264581564861424);
}

#[test]
fn solver_016_test() {
    assert_eq!(Solver016::default().solve(), 1366);

    assert_eq!(Solver016 { n: 0 }.solve(), 1);
    assert_eq!(Solver016 { n: 1 }.solve(), 2);
    assert_eq!(Solver016 { n: 4 }.solve(), 7);
    assert_eq!(Solver016 { n: 8 }.solve(), 13);
    assert_eq!(Solver016 { n: 10 }.solve(), 7);
    assert_eq!(Solver016 { n: 15 }.solve(), 26);
    assert_eq!(Solver016 { n: 33 }.solve(), 62);
    assert_eq!(Solver016 { n: 65 }.solve(), 86);
    assert_eq!(Solver016 { n: 100 }.solve(), 115);
    assert_eq!(Solver016 { n: 200 }.solve(), 256);
    assert_eq!(Solver016 { n: 500 }.solve(), 679);
    assert_eq!(Solver016 { n: 10000 }.solve(), 13561);
}

#[test]
fn solver_017_test() {
    assert_eq!(Solver017::default().solve(), 21124);

    assert_eq!(Solver017 { n: 5 }.solve(), 19);
    assert_eq!(Solver017 { n: 19 }.solve(), 106);
    assert_eq!(Solver017 { n: 19999 }.solve(), 737203);

    // for a more comprehensive set of tests on the algorithm, check the java version
}

#[test]
fn solver_018_test() {
    assert_eq!(Solver018::default().solve(), 1074);

    assert_eq!(Solver018 { n: 1, ..Default::default() }.solve(), 75);
    assert_eq!(Solver018 { n: 2, ..Default::default() }.solve(), 170);
    assert_eq!(Solver018 { n: 10, ..Default::default() }.solve(), 696);
}

#[test]
fn solver_019_test() {
    assert_eq!(Solver019::default().solve(), 171);

    assert_eq!(Solver019 { n: 1 }.solve(), 2);
    assert_eq!(Solver019 { n: 2 }.solve(), 3);
    assert_eq!(Solver019 { n: 3 }.solve(), 6);
    assert_eq!(Solver019 { n: 4 }.solve(), 7);
    assert_eq!(Solver019 { n: 10 }.solve(), 17);
    assert_eq!(Solver019 { n: 10000 }.solve(), 17200);
}

#[test]
fn solver_020_test() {
    assert_eq!(Solver020::default().solve(), 648);

    assert_eq!(Solver020 { n: 1 }.solve(), 1);
    assert_eq!(Solver020 { n: 2 }.solve(), 2);
    assert_eq!(Solver020 { n: 3 }.solve(), 6);
    assert_eq!(Solver020 { n: 4 }.solve(), 6);
    assert_eq!(Solver020 { n: 10 }.solve(), 27);
    assert_eq!(Solver020 { n: 1000 }.solve(), 10539);
}

#[test]
fn solver_021_test() {
    assert_eq!(Solver021::default().solve(), 31626);

    assert_eq!(Solver021 { n: 300 }.solve(), 504);
    assert_eq!(Solver021 { n: 200000 }.solve(), 2896242);
}

#[test]
fn solver_022_test() {
    assert_eq!(Solver022::default().solve(), 871198282);

    assert_eq!(Solver022 { n: 5, ..Default::default() }.solve(), 496);
    assert_eq!(Solver022 { n: 938, ..Default::default() }.solve(), 26819198);

    assert_eq!(Solver022 { n: 1, input: String::from("COLIN") }.solve(), 53);
    assert_eq!(Solver022 { n: 1, input: String::from("LUIS") }.solve(), 61);
    assert_eq!(Solver022 { n: 1, input: String::from("BARREIRO") }.solve(), 86);
    assert_eq!(Solver022 { n: 2, input: String::from("\"LUIS\",\"BARREIRO\"") }.solve(), 208);
}

#[test]
fn solver_023_test() {
    assert_eq!(Solver023::default().solve(), 4179871);

    assert_eq!(Solver023 { n: 23 }.solve(), 276);
    assert_eq!(Solver023 { n: 24 }.solve(), 276);
    assert_eq!(Solver023 { n: 25 }.solve(), 301);
    assert_eq!(Solver023 { n: 50000 }.solve(), 4179871);
}

#[test]
fn solver_024_test() {
    assert_eq!(Solver024::default().solve(), 2783915460);

    assert_eq!(Solver024 { n: 1, ..Default::default() }.solve(), 123456789);
    assert_eq!(Solver024 { n: 2, ..Default::default() }.solve(), 123456798);
    assert_eq!(Solver024 { n: 3628799, ..Default::default() }.solve(), 9876543201);
    assert_eq!(Solver024 { n: 3628800, ..Default::default() }.solve(), 9876543210);

    assert_eq!(Solver024 { n: 1, base: &[0, 1, 2] }.solve(), 12);
    assert_eq!(Solver024 { n: 2, base: &[0, 1, 2] }.solve(), 21);
    assert_eq!(Solver024 { n: 3, base: &[0, 1, 2] }.solve(), 102);
    assert_eq!(Solver024 { n: 4, base: &[0, 1, 2] }.solve(), 120);
    assert_eq!(Solver024 { n: 5, base: &[0, 1, 2] }.solve(), 201);
    assert_eq!(Solver024 { n: 6, base: &[0, 1, 2] }.solve(), 210);
}

#[test]
fn solver_025_test() {
    assert_eq!(Solver025::default().solve(), 4782);

    assert_eq!(Solver025 { n: 1 }.solve(), 2);
    assert_eq!(Solver025 { n: 2 }.solve(), 7);
    assert_eq!(Solver025 { n: 3 }.solve(), 12);
    assert_eq!(Solver025 { n: 8 }.solve(), 36);
    assert_eq!(Solver025 { n: 5000 }.solve(), 23922);
    assert_eq!(Solver025 { n: 50000 }.solve(), 239246);
}

#[test]
fn solver_026_test() {
    assert_eq!(Solver026::default().solve(), 983);

    assert_eq!(Solver026 { n: 10 }.solve(), 7);
    assert_eq!(Solver026 { n: 100 }.solve(), 97);
    assert_eq!(Solver026 { n: 10000 }.solve(), 9967);
    assert_eq!(Solver026 { n: 100000 }.solve(), 99989);
}

#[test]
fn solver_027_test() {
    assert_eq!(Solver027::default().solve(), -59231);

    assert_eq!(Solver027 { n: 1700 }.solve(), -126479);
    assert_eq!(Solver027 { n: 45 }.solve(), -129);
}

#[test]
fn solver_028_test() {
    assert_eq!(Solver028::default().solve(), 669171001);

    assert_eq!(Solver028 { n: 3 }.solve(), 25);
    assert_eq!(Solver028 { n: 5 }.solve(), 101);
}

#[test]
fn solver_029_test() {
    assert_eq!(Solver029::default().solve(), 9183);

    assert_eq!(Solver029 { n: 2 }.solve(), 1);
    assert_eq!(Solver029 { n: 3 }.solve(), 4);
    assert_eq!(Solver029 { n: 4 }.solve(), 8);
    assert_eq!(Solver029 { n: 5 }.solve(), 15);
    assert_eq!(Solver029 { n: 6 }.solve(), 23);
    assert_eq!(Solver029 { n: 8 }.solve(), 44);
    assert_eq!(Solver029 { n: 10 }.solve(), 69);
    assert_eq!(Solver029 { n: 20 }.solve(), 324);
    assert_eq!(Solver029 { n: 40 }.solve(), 1365);
    assert_eq!(Solver029 { n: 50 }.solve(), 2184);
}

#[test]
fn solver_030_test() {
    assert_eq!(Solver030::default().solve(), 443839);

    assert_eq!(Solver030 { n: 4 }.solve(), 19316);
}

#[test]
fn solver_031_test() {
    assert_eq!(Solver031::default().solve(), 73682);

    assert_eq!(Solver031 { n: 1, ..Default::default() }.solve(), 1);
    assert_eq!(Solver031 { n: 2, ..Default::default() }.solve(), 2);
    assert_eq!(Solver031 { n: 5, ..Default::default() }.solve(), 4);
    assert_eq!(Solver031 { n: 10, ..Default::default() }.solve(), 11);
    assert_eq!(Solver031 { n: 1000, ..Default::default() }.solve(), 321335886);
    assert_eq!(Solver031 { n: 5000, ..Default::default() }.solve(), 10082315214426);
}

#[test]
fn solver_032_test() {
    assert_eq!(Solver032::default().solve(), 45228);

    assert_eq!(Solver032 { n: 3, ..Default::default() }.solve(), 0);
    assert_eq!(Solver032 { n: 5, ..Default::default() }.solve(), 52);
    assert_eq!(Solver032 { n: 6, ..Default::default() }.solve(), 162);
    assert_eq!(Solver032 { n: 7, ..Default::default() }.solve(), 0);
    assert_eq!(Solver032 { n: 8, ..Default::default() }.solve(), 13458);
    //assert_eq!(Solver032 { n: 10, ..Default::default() }.solve(), 602220);
}

#[test]
fn solver_033_test() {
    assert_eq!(Solver033::default().solve(), 100);

    assert_eq!(Solver033 { n: 1, ..Default::default() }.solve(), 1);
    assert_eq!(Solver033 { n: 65, ..Default::default() }.solve(), 4);
    assert_eq!(Solver033 { n: 66, ..Default::default() }.solve(), 10);
    assert_eq!(Solver033 { n: 1000, ..Default::default() }.solve(), 100);
}

#[test]
fn solver_034_test() {
    assert_eq!(Solver034::default().solve(), 40730);

    assert_eq!(Solver034 { n: 145, ..Default::default() }.solve(), 145);
    assert_eq!(Solver034 { n: 100000, ..Default::default() }.solve(), 40730);
}

#[test]
fn solver_035_test() {
    assert_eq!(Solver035::default().solve(), 55);

    assert_eq!(Solver035 { n: 5, ..Default::default() }.solve(), 2);
    assert_eq!(Solver035 { n: 9, ..Default::default() }.solve(), 4);
    assert_eq!(Solver035 { n: 20, ..Default::default() }.solve(), 7);
    assert_eq!(Solver035 { n: 100, ..Default::default() }.solve(), 13);
    assert_eq!(Solver035 { n: 100000, ..Default::default() }.solve(), 43);
    assert_eq!(Solver035 { n: 10000000, ..Default::default() }.solve(), 55);
}

#[test]
fn solver_036_test() {
    assert_eq!(Solver036::default().solve(), 872187);

    assert_eq!(Solver036 { n: 2, ..Default::default() }.solve(), 1);
    assert_eq!(Solver036 { n: 20, ..Default::default() }.solve(), 25);
    assert_eq!(Solver036 { n: 586, ..Default::default() }.solve(), 1055);
    assert_eq!(Solver036 { n: 10000000, ..Default::default() }.solve(), 25846868);
}

#[test]
fn solver_037_test() {
    assert_eq!(Solver037::default().solve(), 748317);

    assert_eq!(Solver037 { n: 1, ..Default::default() }.solve(), 23);
    assert_eq!(Solver037 { n: 4, ..Default::default() }.solve(), 186);
    assert_eq!(Solver037 { n: 9, ..Default::default() }.solve(), 5123);
    assert_eq!(Solver037 { n: 10, ..Default::default() }.solve(), 8920);
}

#[test]
fn solver_038_test() {
    assert_eq!(Solver038::default().solve(), 932718654);

    assert_eq!(Solver038 { n: 7, ..Default::default() }.solve(), 0);
    assert_eq!(Solver038 { n: 8, ..Default::default() }.solve(), 78156234);
    assert_eq!(Solver038 { n: 10, ..Default::default() }.solve(), 0);
    assert_eq!(Solver038 { n: 15, ..Default::default() }.solve(), 0);
}

#[test]
fn solver_039_test() {
    assert_eq!(Solver039::default().solve(), 840);

    assert_eq!(Solver039 { n: 14, ..Default::default() }.solve(), 12);
    assert_eq!(Solver039 { n: 66, ..Default::default() }.solve(), 60);
    assert_eq!(Solver039 { n: 150, ..Default::default() }.solve(), 120);
    assert_eq!(Solver039 { n: 10000, ..Default::default() }.solve(), 7560);
}

#[test]
fn solver_040_test() {
    assert_eq!(Solver040::default().solve(), 210);

    assert_eq!(Solver040 { n: 1, ..Default::default() }.solve(), 1);
    assert_eq!(Solver040 { n: 2, ..Default::default() }.solve(), 1);
    assert_eq!(Solver040 { n: 3, ..Default::default() }.solve(), 5);
    assert_eq!(Solver040 { n: 4, ..Default::default() }.solve(), 15);
    assert_eq!(Solver040 { n: 5, ..Default::default() }.solve(), 105);
    assert_eq!(Solver040 { n: 10, ..Default::default() }.solve(), 11760);
    assert_eq!(Solver040 { n: 12, ..Default::default() }.solve(), 0);
}
