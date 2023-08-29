// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::io::load_default_test_data;
use algorithm::long::{heptagonal, hexagonal, pentagonal, triangle};
use algorithm::root::square;
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
    // assert_eq!(Solver004 { n: 5 }.solve(), 9966006699);
}

#[test]
fn solver_005_test() {
    assert_eq!(Solver005::default().solve(), 232792560);

    assert_eq!(Solver005 { n: 1 }.solve(), 1);
    assert_eq!(Solver005 { n: 2 }.solve(), 2);
    assert_eq!(Solver005 { n: 3 }.solve(), 6);
    assert_eq!(Solver005 { n: 4 }.solve(), 12);
    assert_eq!(Solver005 { n: 5 }.solve(), 60);
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
    assert_eq!(Solver009 { n: 20000 }.solve(), 255000000000);
    // assert_eq!(Solver009 { n: 20000 }.solve(), 265387500000);
}

#[test]
fn solver_010_test() {
    assert_eq!(Solver010::default().solve(), 142913828922);

    assert_eq!(Solver010 { n: 5 }.solve(), 5);
    assert_eq!(Solver010 { n: 10 }.solve(), 17);
    assert_eq!(Solver010 { n: 1000000 }.solve(), 37550402023);
    assert_eq!(Solver010 { n: 20000000 }.solve(), 12272577818052);
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

    assert_eq!(Solver012 { n: 2 }.solve(), 3);
    assert_eq!(Solver012 { n: 4 }.solve(), 6);
    assert_eq!(Solver012 { n: 5 }.solve(), 28);
    assert_eq!(Solver012 { n: 10 }.solve(), 120);
    assert_eq!(Solver012 { n: 50 }.solve(), 25200);
    assert_eq!(Solver012 { n: 100 }.solve(), 73920);
    assert_eq!(Solver012 { n: 150 }.solve(), 749700);
    assert_eq!(Solver012 { n: 200 }.solve(), 2031120);
    assert_eq!(Solver012 { n: 600 }.solve(), 103672800);
    // assert_eq!(Solver012 { n: 750 }.solve(), 236215980); // this fails
    // assert_eq!(Solver012 { n: 1000 }.solve(), 842161320);
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

    assert_eq!(Solver024 { n: 1, base: vec![0, 1, 2] }.solve(), 12);
    assert_eq!(Solver024 { n: 2, base: vec![0, 1, 2] }.solve(), 21);
    assert_eq!(Solver024 { n: 3, base: vec![0, 1, 2] }.solve(), 102);
    assert_eq!(Solver024 { n: 4, base: vec![0, 1, 2] }.solve(), 120);
    assert_eq!(Solver024 { n: 5, base: vec![0, 1, 2] }.solve(), 201);
    assert_eq!(Solver024 { n: 6, base: vec![0, 1, 2] }.solve(), 210);
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
    // assert_eq!(Solver031 { n: 5000, ..Default::default() }.solve(), 10082315214426);
}

#[test]
fn solver_032_test() {
    assert_eq!(Solver032::default().solve(), 45228);

    assert_eq!(Solver032 { n: 3 }.solve(), 0);
    assert_eq!(Solver032 { n: 5 }.solve(), 52);
    assert_eq!(Solver032 { n: 6 }.solve(), 162);
    assert_eq!(Solver032 { n: 7 }.solve(), 0);
    assert_eq!(Solver032 { n: 8 }.solve(), 13458);
    // assert_eq!(Solver032 { n: 10, ..Default::default() }.solve(), 602220);
}

#[test]
fn solver_033_test() {
    assert_eq!(Solver033::default().solve(), 100);

    assert_eq!(Solver033 { n: 1 }.solve(), 1);
    assert_eq!(Solver033 { n: 65 }.solve(), 4);
    assert_eq!(Solver033 { n: 66 }.solve(), 10);
    assert_eq!(Solver033 { n: 1000 }.solve(), 100);
}

#[test]
fn solver_034_test() {
    assert_eq!(Solver034::default().solve(), 40730);

    assert_eq!(Solver034 { n: 145 }.solve(), 145);
    assert_eq!(Solver034 { n: 100000 }.solve(), 40730);
}

#[test]
fn solver_035_test() {
    assert_eq!(Solver035::default().solve(), 55);

    assert_eq!(Solver035 { n: 5 }.solve(), 2);
    assert_eq!(Solver035 { n: 9 }.solve(), 4);
    assert_eq!(Solver035 { n: 20 }.solve(), 7);
    assert_eq!(Solver035 { n: 100 }.solve(), 13);
    assert_eq!(Solver035 { n: 100000 }.solve(), 43);
    assert_eq!(Solver035 { n: 2000000 }.solve(), 55);
}

#[test]
fn solver_036_test() {
    assert_eq!(Solver036::default().solve(), 872187);

    assert_eq!(Solver036 { n: 2 }.solve(), 1);
    assert_eq!(Solver036 { n: 20 }.solve(), 25);
    assert_eq!(Solver036 { n: 586 }.solve(), 1055);
    assert_eq!(Solver036 { n: 10000000 }.solve(), 25846868);
}

#[test]
fn solver_037_test() {
    assert_eq!(Solver037::default().solve(), 748317);

    assert_eq!(Solver037 { n: 1 }.solve(), 23);
    assert_eq!(Solver037 { n: 4 }.solve(), 186);
    assert_eq!(Solver037 { n: 9 }.solve(), 5123);
    assert_eq!(Solver037 { n: 10 }.solve(), 8920);
    assert_eq!(Solver037 { n: 11 }.solve(), 748317);
}

#[test]
fn solver_038_test() {
    assert_eq!(Solver038::default().solve(), 932718654);

    assert_eq!(Solver038 { n: 7 }.solve(), 0);
    assert_eq!(Solver038 { n: 8 }.solve(), 78156234);
    assert_eq!(Solver038 { n: 10 }.solve(), 0);
    assert_eq!(Solver038 { n: 14 }.solve(), 0);
}

#[test]
fn solver_039_test() {
    assert_eq!(Solver039::default().solve(), 840);

    assert_eq!(Solver039 { n: 14 }.solve(), 12);
    assert_eq!(Solver039 { n: 66 }.solve(), 60);
    assert_eq!(Solver039 { n: 150 }.solve(), 120);
    assert_eq!(Solver039 { n: 3000 }.solve(), 2520);
    assert_eq!(Solver039 { n: 10000 }.solve(), 9240);
    assert_eq!(Solver039 { n: 100000 }.solve(), 55440);
    assert_eq!(Solver039 { n: 1000000 }.solve(), 720720);
    assert_eq!(Solver039 { n: 10000000 }.solve(), 6126120);
    assert_eq!(Solver039 { n: 100000000 }.solve(), 77597520);
    assert_eq!(Solver039 { n: 1000000000 }.solve(), 892371480);
    assert_eq!(Solver039 { n: 10000000000 }.solve(), 5354228880);
}

#[test]
fn solver_040_test() {
    assert_eq!(Solver040::default().solve(), 210);

    assert_eq!(Solver040 { n: 1 }.solve(), 1);
    assert_eq!(Solver040 { n: 2 }.solve(), 1);
    assert_eq!(Solver040 { n: 3 }.solve(), 5);
    assert_eq!(Solver040 { n: 4 }.solve(), 15);
    assert_eq!(Solver040 { n: 5 }.solve(), 105);
    assert_eq!(Solver040 { n: 10 }.solve(), 11760);
    assert_eq!(Solver040 { n: 12 }.solve(), 0);
}

#[test]
fn solver_041_test() {
    assert_eq!(Solver041::default().solve(), 7652413);

    assert_eq!(Solver041 { n: 4 }.solve(), 4231);
    assert_eq!(Solver041 { n: 7 }.solve(), 7652413);
    // assert_eq!(Solver041 { n: 10, ..Default::default() }.solve(), 3036985741);
}

#[test]
fn solver_042_test() {
    assert_eq!(Solver042::default().solve(), 162);

    assert_eq!(Solver042 { n: 1, ..Default::default() }.solve(), 1);
    assert_eq!(Solver042 { n: 2, ..Default::default() }.solve(), 2);
    assert_eq!(Solver042 { n: 3, ..Default::default() }.solve(), 2);
    assert_eq!(Solver042 { n: 4, ..Default::default() }.solve(), 2);
    assert_eq!(Solver042 { n: 5, ..Default::default() }.solve(), 3);
    assert_eq!(Solver042 { n: 6, ..Default::default() }.solve(), 3);

    assert_eq!(Solver042 { n: 1, input: String::from("SKY") }.solve(), 1);
    assert_eq!(Solver042 { n: 2, input: String::from("\"LUIS\",\"BARREIRO\"") }.solve(), 0);
}

#[test]
fn solver_043_test() {
    assert_eq!(Solver043::default().solve(), 16695334890);

    assert_eq!(Solver043 { n: 4 }.solve(), 711104);
    assert_eq!(Solver043 { n: 7 }.solve(), 1099210170);
    // assert_eq!(Solver043 { n: 10, ..Default::default() }.solve(), 66301145786);
}

#[test]
fn solver_044_test() {
    assert_eq!(Solver044::default().solve(), 5482660);

    // assert_eq!(Solver044 { n: 1 }.solve(), 0);
}

#[test]
fn solver_045_test() {
    assert_eq!(Solver045::default().solve(), 1533776805);

    assert_eq!(Solver045 { n: 2 }.solve(), 40755);
    // assert_eq!(Solver045 { n: 27694 }.solve(), 0);
}

#[test]
fn solver_046_test() {
    assert_eq!(Solver046::default().solve(), 5777);

    assert_eq!(Solver046 { n: 2 }.solve(), 5993);
    // assert_eq!(Solver046 { n: 3 }.solve(), 0);
}

#[test]
fn solver_047_test() {
    assert_eq!(Solver047::default().solve(), 134043);

    assert_eq!(Solver047 { n: 2 }.solve(), 14);
    assert_eq!(Solver047 { n: 3 }.solve(), 644);
    // assert_eq!(Solver047 { n: 5 }.solve(), 129963314);
}

#[test]
fn solver_048_test() {
    assert_eq!(Solver048::default().solve(), 9110846700);

    assert_eq!(Solver048 { n: 2, ..Default::default() }.solve(), 5);
    assert_eq!(Solver048 { n: 10, ..Default::default() }.solve(), 405071317);
    assert_eq!(Solver048 { n: 100, ..Default::default() }.solve(), 9027641920);
    assert_eq!(Solver048 { n: 10000, ..Default::default() }.solve(), 6237204500);
}

#[test]
fn solver_049_test() {
    assert_eq!(Solver049::default().solve(), 296962999629);

    assert_eq!(Solver049 { n: 5 }.solve(), 114831481318143);
    assert_eq!(Solver049 { n: 6 }.solve(), 148171481171814171);
}

#[test]
fn solver_050_test() {
    assert_eq!(Solver050::default().solve(), 997651);

    assert_eq!(Solver050 { n: 2 }.solve(), 41);
    assert_eq!(Solver050 { n: 3 }.solve(), 953);
    assert_eq!(Solver050 { n: 4 }.solve(), 9521);
    assert_eq!(Solver050 { n: 5 }.solve(), 92951);
    assert_eq!(Solver050 { n: 7 }.solve(), 9964597);
    assert_eq!(Solver050 { n: 8 }.solve(), 99819619);
    assert_eq!(Solver050 { n: 9 }.solve(), 999715711);
}

#[test]
fn solver_051_test() {
    assert_eq!(Solver051::default().solve(), 121313);

    assert_eq!(Solver051 { n: 6 }.solve(), 13);
    assert_eq!(Solver051 { n: 7 }.solve(), 56003);
}

#[test]
fn solver_052_test() {
    assert_eq!(Solver052::default().solve(), 142857);

    assert_eq!(Solver052 { n: 3 }.solve(), 142857);
    assert_eq!(Solver052 { n: 4 }.solve(), 142857);
    assert_eq!(Solver052 { n: 5 }.solve(), 142857);
    assert_eq!(Solver052 { n: 7 }.solve(), 14298570);
}

#[test]
fn solver_053_test() {
    assert_eq!(Solver053::default().solve(), 4075);

    assert_eq!(Solver053 { n: 22 }.solve(), 0);
    assert_eq!(Solver053 { n: 23 }.solve(), 4);
    assert_eq!(Solver053 { n: 24 }.solve(), 11);
    assert_eq!(Solver053 { n: 25 }.solve(), 21);
    assert_eq!(Solver053 { n: 50 }.solve(), 692);
    assert_eq!(Solver053 { n: 200 }.solve(), 18461);
    assert_eq!(Solver053 { n: 1000 }.solve(), 494861);
    assert_eq!(Solver053 { n: 1000000 }.solve(), 499997496533);
}

#[test]
fn solver_054_test() {
    assert_eq!(Solver054::default().solve(), 376);

    assert_eq!(Solver054 { n: 1, ..Default::default() }.solve(), 0);
    assert_eq!(Solver054 { n: 10, ..Default::default() }.solve(), 5);
    assert_eq!(Solver054 { n: 50, ..Default::default() }.solve(), 23);
    assert_eq!(Solver054 { n: 100, ..Default::default() }.solve(), 42);
    assert_eq!(Solver054 { n: 200, ..Default::default() }.solve(), 77);
    assert_eq!(Solver054 { n: 500, ..Default::default() }.solve(), 185);
}

#[test]
fn solver_055_test() {
    assert_eq!(Solver055::default().solve(), 249);

    assert_eq!(Solver055 { n: 100 }.solve(), 0);
    assert_eq!(Solver055 { n: 1000 }.solve(), 13);
    assert_eq!(Solver055 { n: 100_000 }.solve(), 6208); // THRESHOLD = 50
}

#[test]
fn solver_056_test() {
    assert_eq!(Solver056::default().solve(), 972);

    assert_eq!(Solver056 { n: 5 }.solve(), 13);
    assert_eq!(Solver056 { n: 10 }.solve(), 45);
    assert_eq!(Solver056 { n: 50 }.solve(), 406);
    assert_eq!(Solver056 { n: 500 }.solve(), 6310);
    assert_eq!(Solver056 { n: 1000 }.solve(), 13888);
}

#[test]
fn solver_057_test() {
    assert_eq!(Solver057::default().solve(), 153);

    assert_eq!(Solver057 { n: 8 }.solve(), 1);
    assert_eq!(Solver057 { n: 100 }.solve(), 15);
    assert_eq!(Solver057 { n: 10_000 }.solve(), 1508);
    assert_eq!(Solver057 { n: 100_000 }.solve(), 15052);
    assert_eq!(Solver057 { n: 1_000_000 }.solve(), 150520);
}

#[test]
fn solver_058_test() {
    assert_eq!(Solver058::default().solve(), 26241);

    assert_eq!(Solver058 { n: 55 }.solve(), 9);
    assert_eq!(Solver058 { n: 50 }.solve(), 11);
    assert_eq!(Solver058 { n: 44 }.solve(), 13);
    assert_eq!(Solver058 { n: 30 }.solve(), 49);
    assert_eq!(Solver058 { n: 20 }.solve(), 309);
    assert_eq!(Solver058 { n: 15 }.solve(), 981);
    assert_eq!(Solver058 { n: 9 }.solve(), 74373);
    // assert_eq!(Solver058 { n: 8 }.solve(), 238733);
    // assert_eq!(Solver058 { n: 7 }.solve(), 1213001);
}

#[test]
fn solver_059_test() {
    assert_eq!(Solver059::default().solve(), 129448);

    assert_eq!(Solver059 { n: 1, ..Default::default() }.solve(), 65);
    assert_eq!(Solver059 { n: 3, ..Default::default() }.solve(), 207);
}

#[test]
fn solver_060_test() {
    assert_eq!(Solver060::default().solve(), 26033);

    assert_eq!(Solver060 { n: 2 }.solve(), 10);
    assert_eq!(Solver060 { n: 3 }.solve(), 107);
    assert_eq!(Solver060 { n: 4 }.solve(), 792);
}

#[test]
fn solver_061_test() {
    assert_eq!(Solver061::default().solve(), 28684);

    assert_eq!(Solver061 { n: 4, input: vec![triangle, square, pentagonal] }.solve(), 19291);
    assert_eq!(Solver061 { n: 4, input: vec![triangle, square, pentagonal, hexagonal] }.solve(), 12524);
    assert_eq!(Solver061 { n: 4, input: vec![triangle, square, pentagonal, hexagonal, heptagonal] }.solve(), 18685);
    assert_eq!(Solver061 { n: 6, ..Default::default() }.solve(), 2240238);
    assert_eq!(Solver061 { n: 8, ..Default::default() }.solve(), 227322730);
}

#[test]
fn solver_062_test() {
    assert_eq!(Solver062::default().solve(), 127035954683);

    assert_eq!(Solver062 { n: 2 }.solve(), 125);
    assert_eq!(Solver062 { n: 3 }.solve(), 41063625);
    assert_eq!(Solver062 { n: 4 }.solve(), 1006012008);
    assert_eq!(Solver062 { n: 6 }.solve(), 1426487591593);
    assert_eq!(Solver062 { n: 7 }.solve(), 12804692354875);
    assert_eq!(Solver062 { n: 8 }.solve(), 10340284735656);
    assert_eq!(Solver062 { n: 9 }.solve(), 13465983902671);
}

#[test]
fn solver_063_test() {
    assert_eq!(Solver063::default().solve(), 49);

    assert_eq!(Solver063 { n: 1 }.solve(), 9);
    assert_eq!(Solver063 { n: 2 }.solve(), 15);
    assert_eq!(Solver063 { n: 3 }.solve(), 20);
    assert_eq!(Solver063 { n: 4 }.solve(), 24);
    assert_eq!(Solver063 { n: 5 }.solve(), 27);
    assert_eq!(Solver063 { n: 9 }.solve(), 36);
    assert_eq!(Solver063 { n: 15 }.solve(), 43);
    assert_eq!(Solver063 { n: 150 }.solve(), 49);
}

#[test]
fn solver_064_test() {
    assert_eq!(Solver064::default().solve(), 1322);

    assert_eq!(Solver064 { n: 2 }.solve(), 1);
    assert_eq!(Solver064 { n: 13 }.solve(), 4);
    assert_eq!(Solver064 { n: 23 }.solve(), 5);
    assert_eq!(Solver064 { n: 100000 }.solve(), 11486);
}

#[test]
fn solver_065_test() {
    assert_eq!(Solver065::default().solve(), 272);

    assert_eq!(Solver065 { n: 1 }.solve(), 2);
    assert_eq!(Solver065 { n: 2 }.solve(), 3);
    assert_eq!(Solver065 { n: 9 }.solve(), 13);
    assert_eq!(Solver065 { n: 10 }.solve(), 17);
    assert_eq!(Solver065 { n: 1000 }.solve(), 4034);
    assert_eq!(Solver065 { n: 10000 }.solve(), 55322);
}

#[test]
fn solver_066_test() {
    assert_eq!(Solver066::default().solve(), 661);

    assert_eq!(Solver066 { n: 30 }.solve(), 29);
    assert_eq!(Solver066 { n: 100 }.solve(), 61);
    assert_eq!(Solver066 { n: 130 }.solve(), 109);
    assert_eq!(Solver066 { n: 1024 }.solve(), 1021);
    assert_eq!(Solver066 { n: 10000 }.solve(), 9949);
    assert_eq!(Solver066 { n: 37000 }.solve(), 36061);
    assert_eq!(Solver066 { n: 100000 }.solve(), 92821);
}

#[test]
fn solver_067_test() {
    assert_eq!(Solver067::default().solve(), 7273);

    assert_eq!(Solver067 { n: 1, ..Default::default() }.solve(), 59);
    assert_eq!(Solver067 { n: 2, ..Default::default() }.solve(), 132);
    assert_eq!(Solver067 { n: 5, ..Default::default() }.solve(), 324);
    assert_eq!(Solver067 { n: 10, ..Default::default() }.solve(), 714);
    assert_eq!(Solver067 { n: 20, ..Default::default() }.solve(), 1407);
    assert_eq!(Solver067 { n: 30, ..Default::default() }.solve(), 2156);
    assert_eq!(Solver067 { n: 50, ..Default::default() }.solve(), 3606);
    assert_eq!(Solver067 { n: 80, ..Default::default() }.solve(), 5774);
}

#[test]
fn solver_068_test() {
    assert_eq!(Solver068::default().solve(), 6531031914842725);

    assert_eq!(Solver068 { n: 3 }.solve(), 432621513);
    // assert_eq!(Solver068 { n: 7 }.solve(), 87414411315125211261063937); // overflow
    // assert_eq!(Solver068 { n: 9 }.solve(), 109518511716166215271473133812841149); // overflow
    // assert_eq!(Solver068 { n: 11 }.solve(), 121162261211720721928188317391694154101410513511); // overflow
    // assert_eq!(Solver068 { n: 13 }.solve(), 141372671251824822329229321310201041941118115175121612615613); // overflow
}

#[test]
fn solver_069_test() {
    assert_eq!(Solver069::default().solve(), 510510);

    assert_eq!(Solver069 { n: 10 }.solve(), 6);
    assert_eq!(Solver069 { n: 40 }.solve(), 30);
    assert_eq!(Solver069 { n: 300 }.solve(), 210);
    assert_eq!(Solver069 { n: 10000000 }.solve(), 9699690);
    assert_eq!(Solver069 { n: 100000000000000000 }.solve(), 13082761331670030);
}

#[test]
fn solver_070_test() {
    assert_eq!(Solver070::default().solve(), 8319823);

    assert_eq!(Solver070 { n: 2 }.solve(), 21);
    assert_eq!(Solver070 { n: 3 }.solve(), 291);
    assert_eq!(Solver070 { n: 4 }.solve(), 4435);
    assert_eq!(Solver070 { n: 5 }.solve(), 75841);
    assert_eq!(Solver070 { n: 6 }.solve(), 783169);
    assert_eq!(Solver070 { n: 8 }.solve(), 99836521);
    assert_eq!(Solver070 { n: 9 }.solve(), 990326167);
}

#[test]
fn solver_071_test() {
    assert_eq!(Solver071::default().solve(), 428570);

    assert_eq!(Solver071 { n: 3 }.solve(), 1);
    assert_eq!(Solver071 { n: 4 }.solve(), 1);
    assert_eq!(Solver071 { n: 8 }.solve(), 2);
    assert_eq!(Solver071 { n: 24 }.solve(), 8);
    assert_eq!(Solver071 { n: 30 }.solve(), 11);
    assert_eq!(Solver071 { n: 1000 }.solve(), 428);
    assert_eq!(Solver071 { n: 100000000000000 }.solve(), 42857142857141);
}


#[test]
fn solver_072_test() {
    assert_eq!(Solver072::default().solve(), 303963552391);

    assert_eq!(Solver072 { n: 1 }.solve(), 0);
    assert_eq!(Solver072 { n: 2 }.solve(), 1);
    assert_eq!(Solver072 { n: 3 }.solve(), 3);
    assert_eq!(Solver072 { n: 8 }.solve(), 21);
    assert_eq!(Solver072 { n: 56 }.solve(), 963);
    assert_eq!(Solver072 { n: 100 }.solve(), 3043);
    assert_eq!(Solver072 { n: 1000 }.solve(), 304191);
    assert_eq!(Solver072 { n: 10000 }.solve(), 30397485);
    assert_eq!(Solver072 { n: 100000 }.solve(), 3039650753);
}

#[test]
fn solver_073_test() {
    assert_eq!(Solver073::default().solve(), 7295372);

    assert_eq!(Solver073 { n: 4 }.solve(), 0);
    assert_eq!(Solver073 { n: 5 }.solve(), 1);
    assert_eq!(Solver073 { n: 8 }.solve(), 3);
    assert_eq!(Solver073 { n: 9 }.solve(), 4);
    assert_eq!(Solver073 { n: 10 }.solve(), 4);
    assert_eq!(Solver073 { n: 1000 }.solve(), 50695);
    assert_eq!(Solver073 { n: 10000 }.solve(), 5066251);
    assert_eq!(Solver073 { n: 100000 }.solve(), 506608484);
}

#[test]
fn solver_074_test() {
    assert_eq!(Solver074::default().solve(), 402);

    assert_eq!(Solver074 { n: 1000 }.solve(), 0);
    assert_eq!(Solver074 { n: 10000 }.solve(), 42);
    assert_eq!(Solver074 { n: 100000 }.solve(), 42);
}

#[test]
fn solver_075_test() {
    assert_eq!(Solver075::default().solve(), 161667);

    assert_eq!(Solver075 { n: 100 }.solve(), 11);
    assert_eq!(Solver075 { n: 1000 }.solve(), 112);
    assert_eq!(Solver075 { n: 10000 }.solve(), 1120);
    assert_eq!(Solver075 { n: 100000 }.solve(), 11013);
    assert_eq!(Solver075 { n: 1000000 }.solve(), 107876);
    assert_eq!(Solver075 { n: 10000000 }.solve(), 1067080);
    // assert_eq!(Solver075 { n: 100000000 }.solve(), 10619384);
}

#[test]
fn solver_076_test() {
    assert_eq!(Solver076::default().solve(), 190569291);

    assert_eq!(Solver076 { n: 5 }.solve(), 6);
    assert_eq!(Solver076 { n: 40 }.solve(), 37337);
    assert_eq!(Solver076 { n: 300 }.solve(), 9253082936723601);
}

#[test]
fn solver_077_test() {
    assert_eq!(Solver077::default().solve(), 71);

    assert_eq!(Solver077 { n: 1 }.solve(), 6);
    assert_eq!(Solver077 { n: 4 }.solve(), 10);
    assert_eq!(Solver077 { n: 10 }.solve(), 15);
    assert_eq!(Solver077 { n: 100 }.solve(), 31);
    assert_eq!(Solver077 { n: 1000 }.solve(), 53);
    assert_eq!(Solver077 { n: 10000 }.solve(), 80);
    assert_eq!(Solver077 { n: 100000 }.solve(), 114);
    assert_eq!(Solver077 { n: 1000000 }.solve(), 154);
    assert_eq!(Solver077 { n: 10000000 }.solve(), 201);
    assert_eq!(Solver077 { n: 100000000 }.solve(), 254);
}

#[test]
fn solver_078_test() {
    assert_eq!(Solver078::default().solve(), 55374);

    assert_eq!(Solver078 { n: 2 }.solve(), 2);
    assert_eq!(Solver078 { n: 3 }.solve(), 3);
    assert_eq!(Solver078 { n: 4 }.solve(), 11);
    assert_eq!(Solver078 { n: 5 }.solve(), 4);
    assert_eq!(Solver078 { n: 6 }.solve(), 9);
    assert_eq!(Solver078 { n: 7 }.solve(), 5);
    assert_eq!(Solver078 { n: 8 }.solve(), 11);
    assert_eq!(Solver078 { n: 9 }.solve(), 14);
    assert_eq!(Solver078 { n: 10 }.solve(), 9);
    assert_eq!(Solver078 { n: 11 }.solve(), 6);
    assert_eq!(Solver078 { n: 12 }.solve(), 21);
    assert_eq!(Solver078 { n: 13 }.solve(), 28);
    assert_eq!(Solver078 { n: 14 }.solve(), 10);
    assert_eq!(Solver078 { n: 15 }.solve(), 7);
    assert_eq!(Solver078 { n: 100 }.solve(), 74);
    assert_eq!(Solver078 { n: 200 }.solve(), 128);
    assert_eq!(Solver078 { n: 300 }.solve(), 124);
    assert_eq!(Solver078 { n: 400 }.solve(), 149);
    assert_eq!(Solver078 { n: 500 }.solve(), 74);
    assert_eq!(Solver078 { n: 550 }.solve(), 74);
    assert_eq!(Solver078 { n: 1000 }.solve(), 449);
    assert_eq!(Solver078 { n: 10000 }.solve(), 599);
    assert_eq!(Solver078 { n: 100000 }.solve(), 11224);
    // assert_eq!(Solver078 { n: 10000000 }.solve(), 3099324);
}

#[test]
fn solver_079_test() {
    assert_eq!(Solver079::default().solve(), 73162890);

    // assert_eq!(Solver079 { n: 45, ..Default::default() }.solve(), 73162890);
}

#[test]
fn solver_080_test() {
    assert_eq!(Solver080::default().solve(), 40886);

    assert_eq!(Solver080 { n: 2 }.solve(), 475);
    assert_eq!(Solver080 { n: 3 }.solve(), 916);
    assert_eq!(Solver080 { n: 50 }.solve(), 19543);
    assert_eq!(Solver080 { n: 1000 }.solve(), 435035);
}

#[test]
fn solver_081_test() {
    assert_eq!(Solver081::default().solve(), 427337);

    assert_eq!(Solver081 { n: 2, ..Default::default() }.solve(), 5561);
    assert_eq!(Solver081 { n: 10, ..Default::default() }.solve(), 57442);
    assert_eq!(Solver081 { n: 50, ..Default::default() }.solve(), 261333);

    assert_eq!(Solver081 { n: 5, input: "131,673,234,103,18\n201,96,342,965,150\n630,803,746,422,111\n537,699,497,121,956\n805,732,524,37,331".into() }.solve(), 2427);
}

#[test]
fn solver_082_test() {
    assert_eq!(Solver082::default().solve(), 260324);

    assert_eq!(Solver082 { n: 2, ..Default::default() }.solve(), 1116);
    assert_eq!(Solver082 { n: 10, ..Default::default() }.solve(), 27590);
    assert_eq!(Solver082 { n: 50, ..Default::default() }.solve(), 151952);

    assert_eq!(Solver082 { n: 5, input: "131,673,234,103,18\n201,96,342,965,150\n630,803,746,422,111\n537,699,497,121,956\n805,732,524,37,331".into() }.solve(), 994);
}

#[test]
fn solver_083_test() {
    assert_eq!(Solver083::default().solve(), 425185);

    assert_eq!(Solver083 { n: 2, ..Default::default() }.solve(), 5561);
    assert_eq!(Solver083 { n: 10, ..Default::default() }.solve(), 57442);
    assert_eq!(Solver083 { n: 50, ..Default::default() }.solve(), 259181);

    assert_eq!(Solver083 { n: 5, input: "131,673,234,103,18\n201,96,342,965,150\n630,803,746,422,111\n537,699,497,121,956\n805,732,524,37,331".into() }.solve(), 2297);
}

#[test]
fn solver_084_test() {
    assert_eq!(Solver084::default().solve(), 101524);

    assert_eq!(Solver084 { n: 6 }.solve(), 102400);
    assert_eq!(Solver084 { n: 5 }.solve(), 102425);
    assert_eq!(Solver084 { n: 3 }.solve(), 101415);
    assert_eq!(Solver084 { n: 2 }.solve(), 101316);
    assert_eq!(Solver084 { n: 1 }.solve(), 101214);
}

#[test]
fn solver_085_test() {
    assert_eq!(Solver085::default().solve(), 2772);

    assert_eq!(Solver085 { n: 18 }.solve(), 6);
    assert_eq!(Solver085 { n: 20 }.solve(), 6);
    assert_eq!(Solver085 { n: 100 }.solve(), 16);
    assert_eq!(Solver085 { n: 500000000 }.solve(), 43902);
    assert_eq!(Solver085 { n: 2000000000 }.solve(), 86595);
    assert_eq!(Solver085 { n: 2000000000000 }.solve(), 2309400);
}

#[test]
fn solver_086_test() {
    assert_eq!(Solver086::default().solve(), 1818);

    assert_eq!(Solver086 { n: 1000 }.solve(), 72);
    assert_eq!(Solver086 { n: 2000 }.solve(), 100);
    assert_eq!(Solver086 { n: 10000 }.solve(), 210);
    assert_eq!(Solver086 { n: 100000 }.solve(), 616);
    assert_eq!(Solver086 { n: 10000000 }.solve(), 5400);
    assert_eq!(Solver086 { n: 100000000 }.solve(), 16170);
    // assert_eq!(Solver086 { n: 1000000000 }.solve(), 48655);
    // assert_eq!(Solver086 { n: 10000000000 }.solve(), 147015);
    // assert_eq!(Solver086 { n: 100000000000 }.solve(), 445905);
    // assert_eq!(Solver086 { n: 1000000000000 }.solve(), 1356600);
    // assert_eq!(Solver086 { n: 10000000000000 }.solve(), 4138565);
    // assert_eq!(Solver086 { n: 1000000000000000 }.solve(), 12654786);
    // assert_eq!(Solver086 { n: 10000000000000000 }.solve(), 38775191);
}

#[test]
fn solver_087_test() {
    assert_eq!(Solver087::default().solve(), 1097343);

    assert_eq!(Solver087 { n: 46 }.solve(), 2);
    assert_eq!(Solver087 { n: 47 }.solve(), 2);
    assert_eq!(Solver087 { n: 48 }.solve(), 3);
    assert_eq!(Solver087 { n: 49 }.solve(), 3);
    assert_eq!(Solver087 { n: 50 }.solve(), 4);
    assert_eq!(Solver087 { n: 5000 }.solve(), 395);
    assert_eq!(Solver087 { n: 500000 }.solve(), 18899);
    // assert_eq!(Solver087 { n: 1000000000 }.solve(), 16888551);
}

#[test]
fn solver_088_test() {
    assert_eq!(Solver088::default().solve(), 7587457);

    assert_eq!(Solver088 { n: 6 }.solve(), 30);
    assert_eq!(Solver088 { n: 12 }.solve(), 61);
    assert_eq!(Solver088 { n: 120 }.solve(), 2940);
    assert_eq!(Solver088 { n: 1200 }.solve(), 125128);
    assert_eq!(Solver088 { n: 120000 }.solve(), 479495455);
    // assert_eq!(Solver088 { n: 1200000 }.solve(), 32349165128);
    // assert_eq!(Solver088 { n: 12000000 }.solve(), 2257497330648);
}

#[test]
fn solver_089_test() {
    assert_eq!(Solver089::default().solve(), 743);

    assert_eq!(Solver089 { n: 3, ..Default::default() }.solve(), 3);
    assert_eq!(Solver089 { n: 100, ..Default::default() }.solve(), 62);

    assert_eq!(Solver089 { n: 6, input: "IIIIIIIIIIIIIIII\nVIIIIIIIIIII\nVVIIIIII\nXIIIIII\nVVVI\nXVI".into() }.solve(), 32);
}

#[test]
fn solver_090_test() {
    assert_eq!(Solver090::default().solve(), 1217);

    assert_eq!(Solver090 { n: 1 }.solve(), 55);
    assert_eq!(Solver090 { n: 3 }.solve(), 9600);
    // assert_eq!(Solver090 { n: 4 }.solve(), 29364);
}

#[test]
fn solver_091_test() {
    assert_eq!(Solver091::default().solve(), 14234);

    assert_eq!(Solver091 { n: 1 }.solve(), 3);
    assert_eq!(Solver091 { n: 2 }.solve(), 14);
    assert_eq!(Solver091 { n: 3 }.solve(), 33);
    assert_eq!(Solver091 { n: 4 }.solve(), 62);
    assert_eq!(Solver091 { n: 5 }.solve(), 101);
    assert_eq!(Solver091 { n: 10 }.solve(), 448);
    assert_eq!(Solver091 { n: 100 }.solve(), 62848);
    assert_eq!(Solver091 { n: 500 }.solve(), 1923928);
    assert_eq!(Solver091 { n: 1000 }.solve(), 8318030);
    assert_eq!(Solver091 { n: 5000 }.solve(), 244468764);
    // assert_eq!(Solver091 { n: 10000 }.solve(), 1040912564);
    // assert_eq!(Solver091 { n: 50000 }.solve(), 29688424644);
    // assert_eq!(Solver091 { n: 100000 }.solve(), 125072732374);
}

#[test]
fn solver_092_test() {
    assert_eq!(Solver092::default().solve(), 8581146);

    assert_eq!(Solver092 { n: 1 }.solve(), 7);
    assert_eq!(Solver092 { n: 2 }.solve(), 80);
    assert_eq!(Solver092 { n: 3 }.solve(), 857);
    assert_eq!(Solver092 { n: 4 }.solve(), 8558);
    assert_eq!(Solver092 { n: 5 }.solve(), 85623);
    assert_eq!(Solver092 { n: 6 }.solve(), 856929);
    assert_eq!(Solver092 { n: 10 }.solve(), 8507390852);
    assert_eq!(Solver092 { n: 15 }.solve(), 869339034137667);
    assert_eq!(Solver092 { n: 19 }.solve(), 8816770037940618762);
}

#[test]
fn solver_093_test() {
    assert_eq!(Solver093::default().solve(), 1258);

    assert_eq!(Solver093 { n: 2 }.solve(), 12);
    assert_eq!(Solver093 { n: 3 }.solve(), 124);
    assert_eq!(Solver093 { n: 5 }.solve(), 45678);
    // assert_eq!(Solver093 { n: 6 }.solve(), 256789);
}

#[test]
fn solver_094_test() {
    assert_eq!(Solver094::default().solve(), 518408346);

    assert_eq!(Solver094 { n: 100 }.solve(), 66);
    assert_eq!(Solver094 { n: 1000 }.solve(), 984);
    assert_eq!(Solver094 { n: 10000 }.solve(), 3688);
    assert_eq!(Solver094 { n: 100000 }.solve(), 51406);
    assert_eq!(Solver094 { n: 1000000 }.solve(), 716032);
    assert_eq!(Solver094 { n: 10000000 }.solve(), 9973078);
    assert_eq!(Solver094 { n: 100000000 }.solve(), 37220040);
}

#[test]
fn solver_095_test() {
    assert_eq!(Solver095::default().solve(), 14316);

    assert_eq!(Solver095 { n: 10 }.solve(), 6);
    assert_eq!(Solver095 { n: 100 }.solve(), 28);
    assert_eq!(Solver095 { n: 1000 }.solve(), 220);
    assert_eq!(Solver095 { n: 100000 }.solve(), 12496);
}

#[test]
fn solver_096_test() {
    assert_eq!(Solver096::default().solve(), 24702);

    assert_eq!(Solver096 { n: 1, ..Default::default() }.solve(), 483);
    assert_eq!(Solver096 { n: 10, input: load_default_test_data(96) }.solve(), 6765);
    // assert_eq!(Solver096 { n: 375, input: load_default_test_data(96) }.solve(), 90611);
}

#[test]
fn solver_097_test() {
    assert_eq!(Solver097::default().solve(), 8739992577);

    assert_eq!(Solver097 { n: 1 }.solve(), 7);
    assert_eq!(Solver097 { n: 5 }.solve(), 92577);
    assert_eq!(Solver097 { n: 15 }.solve(), 790198739992577);
    assert_eq!(Solver097 { n: 18 }.solve(), 994790198739992577);
}

#[test]
fn solver_098_test() {
    assert_eq!(Solver098::default().solve(), 18769);

    assert_eq!(Solver098 { n: 210, ..Default::default() }.solve(), 0);
    assert_eq!(Solver098 { n: 220, ..Default::default() }.solve(), 18769);

    assert_eq!(Solver098 { n: 2, input: String::from("\"CARE\",\"RACE\"") }.solve(), 9216);
}

#[test]
fn solver_099_test() {
    assert_eq!(Solver099::default().solve(), 709);

    assert_eq!(Solver099 { n: 2, ..Default::default() }.solve(), 2);
}

#[test]
fn solver_100_test() {
    assert_eq!(Solver100::default().solve(), 756872327473);

    assert_eq!(Solver100 { n: 1 }.solve(), 15);
    assert_eq!(Solver100 { n: 2 }.solve(), 85);
    assert_eq!(Solver100 { n: 3 }.solve(), 2871);
    assert_eq!(Solver100 { n: 4 }.solve(), 16731);
    assert_eq!(Solver100 { n: 10 }.solve(), 22280241075);
    assert_eq!(Solver100 { n: 18 }.solve(), 1007937474707144521);
}

#[test]
fn solver_101_test() {
    assert_eq!(Solver101::default().solve(), 37076114526);

    assert_eq!(Solver101 { n: 1, ..Default::default() }.solve(), 1);
    assert_eq!(Solver101 { n: 2, ..Default::default() }.solve(), 1366);
    assert_eq!(Solver101 { n: 3, ..Default::default() }.solve(), 132179);
    assert_eq!(Solver101 { n: 4, ..Default::default() }.solve(), 3224632);
    assert_eq!(Solver101 { n: 5, ..Default::default() }.solve(), 35965583);

    assert_eq!(Solver101 { n: 1, input: vec![1, 0, 0, 0] }.solve(), 1);
    assert_eq!(Solver101 { n: 2, input: vec![1, 0, 0, 0] }.solve(), 16);
    assert_eq!(Solver101 { n: 3, input: vec![1, 0, 0, 0] }.solve(), 74);
}

#[test]
fn solver_102_test() {
    assert_eq!(Solver102::default().solve(), 228);

    assert_eq!(Solver102 { n: 1, ..Default::default() }.solve(), 1);
    assert_eq!(Solver102 { n: 2, ..Default::default() }.solve(), 1);
    assert_eq!(Solver102 { n: 3, ..Default::default() }.solve(), 1);
    assert_eq!(Solver102 { n: 4, ..Default::default() }.solve(), 2);
    assert_eq!(Solver102 { n: 5, ..Default::default() }.solve(), 2);
}

#[test]
fn solver_103_test() {
    assert_eq!(Solver103::default().solve(), 20313839404245);

    assert_eq!(Solver103 { n: 1 }.solve(), 1);
    assert_eq!(Solver103 { n: 2 }.solve(), 12);
    assert_eq!(Solver103 { n: 3 }.solve(), 234);
    assert_eq!(Solver103 { n: 4 }.solve(), 3567);
    assert_eq!(Solver103 { n: 5 }.solve(), 69111213);
    assert_eq!(Solver103 { n: 6 }.solve(), 111819202225);
    assert_eq!(Solver103 { n: 8 }.solve(), 3959707778798184);
    // assert_eq!(Solver103 { n: 9 }.solve(), 78117137148155156157159162); // overflow
    // assert_eq!(Solver103 { n: 10 }.solve(), 155233272292303310311312314317); // overflow
    // assert_eq!(Solver103 { n: 11 }.solve(), 310465543582602613620621622624627); //overflow
}

#[test]
fn solver_104_test() {
    assert_eq!(Solver104::default().solve(), 329468);

    assert_eq!(Solver104 { n: 3 }.solve(), 8999);
    assert_eq!(Solver104 { n: 5 }.solve(), 503214);
}

#[test]
fn solver_105_test() {
    assert_eq!(Solver105::default().solve(), 73702);

    assert_eq!(Solver105 { n: 1, ..Default::default() }.solve(), 0);
    assert_eq!(Solver105 { n: 2, ..Default::default() }.solve(), 1286);
    assert_eq!(Solver105 { n: 3, ..Default::default() }.solve(), 1286);
    assert_eq!(Solver105 { n: 4, ..Default::default() }.solve(), 1286);
    assert_eq!(Solver105 { n: 5, ..Default::default() }.solve(), 1286);
    assert_eq!(Solver105 { n: 6, ..Default::default() }.solve(), 1286);
    assert_eq!(Solver105 { n: 7, ..Default::default() }.solve(), 4599);
}

#[test]
fn solver_106_test() {
    assert_eq!(Solver106::default().solve(), 21384);

    assert_eq!(Solver106 { n: 3 }.solve(), 0);
    assert_eq!(Solver106 { n: 4 }.solve(), 1);
    assert_eq!(Solver106 { n: 5 }.solve(), 5);
    assert_eq!(Solver106 { n: 6 }.solve(), 20);
    assert_eq!(Solver106 { n: 7 }.solve(), 70);
    assert_eq!(Solver106 { n: 8 }.solve(), 231);
    assert_eq!(Solver106 { n: 10 }.solve(), 2289);
    assert_eq!(Solver106 { n: 16 }.solve(), 1744847);
    assert_eq!(Solver106 { n: 32 }.solve(), 65623676689175);
}

#[test]
fn solver_107_test() {
    assert_eq!(Solver107::default().solve(), 259679);

    assert_eq!(Solver107 { n: 5, ..Default::default() }.solve(), 668);
    assert_eq!(Solver107 { n: 4, input: load_default_test_data(107) }.solve(), 49);
    assert_eq!(Solver107 { n: 7, input: load_default_test_data(107) }.solve(), 150);
}

#[test]
fn solver_108_test() {
    assert_eq!(Solver108::default().solve(), 180180);

    assert_eq!(Solver108 { n: 3 }.solve(), 6);
    assert_eq!(Solver108 { n: 100 }.solve(), 1260);
    assert_eq!(Solver108 { n: 200 }.solve(), 4620);
    assert_eq!(Solver108 { n: 500 }.solve(), 55440);
}

#[test]
fn solver_109_test() {
    assert_eq!(Solver109::default().solve(), 38182);

    assert_eq!(Solver109 { n: 7 }.solve(), 22);
    assert_eq!(Solver109 { n: 180 }.solve(), 42336);
}

#[test]
fn solver_110_test() {
    assert_eq!(Solver110::default().solve(), 9350130049860600);
}

#[test]
fn solver_111_test() {
    assert_eq!(Solver111::default().solve(), 612407567715);

    assert_eq!(Solver111 { n: 3 }.solve(), 31283);
    assert_eq!(Solver111 { n: 4 }.solve(), 273700);
    assert_eq!(Solver111 { n: 5 }.solve(), 6045857);
    assert_eq!(Solver111 { n: 6 }.solve(), 43552775);
    assert_eq!(Solver111 { n: 7 }.solve(), 1266769793);
    assert_eq!(Solver111 { n: 8 }.solve(), 4060851254);
    assert_eq!(Solver111 { n: 9 }.solve(), 45609504098);
    assert_eq!(Solver111 { n: 17 }.solve(), 9102804193186049011);
}

#[test]
fn solver_112_test() {
    assert_eq!(Solver112::default().solve(), 1587000);

    assert_eq!(Solver112 { n: 50 }.solve(), 538);
    assert_eq!(Solver112 { n: 90 }.solve(), 21780);
    assert_eq!(Solver112 { n: 98 }.solve(), 377650);
}

#[test]
fn solver_113_test() {
    assert_eq!(Solver113::default().solve(), 51161058134250);

    assert_eq!(Solver113 { n: 6, ..Default::default() }.solve(), 12951);
    assert_eq!(Solver113 { n: 10, ..Default::default() }.solve(), 277032);
    assert_eq!(Solver113 { n: 350, ..Default::default() }.solve(), 9128312112077549750);
}

#[test]
fn solver_114_test() {
    assert_eq!(Solver114::default().solve(), 16475640049);

    assert_eq!(Solver114 { n: 7, ..Default::default() }.solve(), 17);
    assert_eq!(Solver114 { n: 10, ..Default::default() }.solve(), 72);
    assert_eq!(Solver114 { n: 20, ..Default::default() }.solve(), 8855);
    assert_eq!(Solver114 { n: 90, ..Default::default() }.solve(), 3770056902373173215);
}

#[test]
fn solver_115_test() {
    assert_eq!(Solver115::default().solve(), 168);

    assert_eq!(Solver115 { n: 3, ..Default::default() }.solve(), 30);
    assert_eq!(Solver115 { n: 10, ..Default::default() }.solve(), 57);
    assert_eq!(Solver115 { n: 20, ..Default::default() }.solve(), 88);
    assert_eq!(Solver115 { n: 100, ..Default::default() }.solve(), 269);
    assert_eq!(Solver115 { n: 500, ..Default::default() }.solve(), 1066);
}

#[test]
fn solver_116_test() {
    assert_eq!(Solver116::default().solve(), 20492570929);

    assert_eq!(Solver116 { n: 5, ..Default::default() }.solve(), 12);
    assert_eq!(Solver116 { n: 10, ..Default::default() }.solve(), 128);
    assert_eq!(Solver116 { n: 20, ..Default::default() }.solve(), 12566);
    assert_eq!(Solver116 { n: 90, ..Default::default() }.solve(), 4660582127692147865);
}

#[test]
fn solver_117_test() {
    assert_eq!(Solver117::default().solve(), 100808458960497);

    assert_eq!(Solver117 { n: 5, ..Default::default() }.solve(), 15);
    assert_eq!(Solver117 { n: 10, ..Default::default() }.solve(), 401);
    assert_eq!(Solver117 { n: 20, ..Default::default() }.solve(), 283953);
    assert_eq!(Solver117 { n: 66, ..Default::default() }.solve(), 3661260108881759077);
}

#[test]
fn solver_118_test() {
    assert_eq!(Solver118::default().solve(), 44680);

    assert_eq!(Solver118 { n: 3 }.solve(), 2);
    assert_eq!(Solver118 { n: 4 }.solve(), 9);
    assert_eq!(Solver118 { n: 5 }.solve(), 24);
    assert_eq!(Solver118 { n: 6 }.solve(), 77);
    assert_eq!(Solver118 { n: 7 }.solve(), 1205);
    assert_eq!(Solver118 { n: 8 }.solve(), 4076);
}

#[test]
fn solver_119_test() {
    assert_eq!(Solver119::default().solve(), 248155780267521);

    assert_eq!(Solver119 { n: 1 }.solve(), 81);
    assert_eq!(Solver119 { n: 2 }.solve(), 512);
    assert_eq!(Solver119 { n: 5 }.solve(), 5832);
    assert_eq!(Solver119 { n: 10 }.solve(), 614656);
    assert_eq!(Solver119 { n: 20 }.solve(), 24794911296);
}

#[test]
fn solver_120_test() {
    assert_eq!(Solver120::default().solve(), 333082500);

    assert_eq!(Solver120 { n: 7 }.solve(), 100);
    assert_eq!(Solver120 { n: 100 }.solve(), 330750);
    assert_eq!(Solver120 { n: 10000 }.solve(), 333308325000);
    assert_eq!(Solver120 { n: 1000000 }.solve(), 333333083332500000);
}

#[test]
fn solver_121_test() {
    assert_eq!(Solver121::default().solve(), 2269);
    
    assert_eq!(Solver121 { n: 1 }.solve(), 2);
    assert_eq!(Solver121 { n: 2 }.solve(), 6);
    assert_eq!(Solver121 { n: 3 }.solve(), 3);
    assert_eq!(Solver121 { n: 4 }.solve(), 10);
    assert_eq!(Solver121 { n: 7 }.solve(), 17);
    assert_eq!(Solver121 { n: 10 }.solve(), 225);
    assert_eq!(Solver121 { n: 19 }.solve(), 47601);
}

#[test]
fn solver_122_test() {
    assert_eq!(Solver122::default().solve(), 1582);

    assert_eq!(Solver122 { n: 3 }.solve(), 3);
    assert_eq!(Solver122 { n: 15 }.solve(), 50);
    assert_eq!(Solver122 { n: 100 }.solve(), 664);
    assert_eq!(Solver122 { n: 1000 }.solve(), 10808);
}

#[test]
fn solver_123_test() {
    assert_eq!(Solver123::default().solve(), 21035);

    assert_eq!(Solver123 { n: 3 }.solve(), 13);
    assert_eq!(Solver123 { n: 5 }.solve(), 99);
    assert_eq!(Solver123 { n: 9 }.solve(), 7037);
    assert_eq!(Solver123 { n: 12 }.solve(), 191041);
    // assert_eq!(Solver123 { n: 14 }.solve(), 1762729);
    // assert_eq!(Solver123 { n: 15 }.solve(), 5378611);
}

#[test]
fn solver_124_test() {
    assert_eq!(Solver124::default().solve(), 21417);

    assert_eq!(Solver124 { n: 1, ceil: 10 }.solve(), 1);
    assert_eq!(Solver124 { n: 4, ceil: 10 }.solve(), 8);
    assert_eq!(Solver124 { n: 6, ceil: 10 }.solve(), 9);
    assert_eq!(Solver124 { n: 10, ceil: 100 }.solve(), 27);
    assert_eq!(Solver124 { n: 100, ceil: 1000 }.solve(), 176);
    assert_eq!(Solver124 { n: 1000000, ceil: 10000000 }.solve(), 6204480);
}

#[test]
fn solver_125_test() {
    assert_eq!(Solver125::default().solve(), 2906969179);

    assert_eq!(Solver125 { n: 1 }.solve(), 5);
    assert_eq!(Solver125 { n: 2 }.solve(), 137);
    assert_eq!(Solver125 { n: 3 }.solve(), 4164);
    assert_eq!(Solver125 { n: 4 }.solve(), 20268);
    assert_eq!(Solver125 { n: 5 }.solve(), 667526);
    assert_eq!(Solver125 { n: 6 }.solve(), 14893023);
    assert_eq!(Solver125 { n: 7 }.solve(), 303800127);
    assert_eq!(Solver125 { n: 10 }.solve(), 593968508531);
}
