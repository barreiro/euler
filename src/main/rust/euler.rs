// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

pub use euler::*;

/// solves all the problems! (the Euler ones)
#[allow(clippy::too_many_lines)]
pub fn main() {
    let solve = |n, solver: &dyn Solver| {
        let now = std::time::Instant::now();
        println!("Solution for problem {:03} is {:16} ( took {:9.03} ms )", n, solver.solve(), now.elapsed().as_secs_f64() * 1000.0);
    };

    solve(1, &Solver001::default());
    solve(2, &Solver002::default());
    solve(3, &Solver003::default());
    solve(4, &Solver004::default());
    solve(5, &Solver005::default());
    solve(6, &Solver006::default());
    solve(7, &Solver007::default());
    solve(8, &Solver008::default());
    solve(9, &Solver009::default());
    solve(10, &Solver010::default());
    solve(11, &Solver011::default());
    solve(12, &Solver012::default());
    solve(13, &Solver013::default());
    solve(14, &Solver014::default());
    solve(15, &Solver015::default());
    solve(16, &Solver016::default());
    solve(17, &Solver017::default());
    solve(18, &Solver018::default());
    solve(19, &Solver019::default());
    solve(20, &Solver020::default());
    solve(21, &Solver021::default());
    solve(22, &Solver022::default());
    solve(23, &Solver023::default());
    solve(24, &Solver024::default());
    solve(25, &Solver025::default());
    solve(26, &Solver026::default());
    solve(27, &Solver027::default());
    solve(28, &Solver028::default());
    solve(29, &Solver029::default());
    solve(30, &Solver030::default());
    solve(31, &Solver031::default());
    solve(32, &Solver032::default());
    solve(33, &Solver033::default());
    solve(34, &Solver034::default());
    solve(35, &Solver035::default());
    solve(36, &Solver036::default());
    solve(37, &Solver037::default());
    solve(38, &Solver038::default());
    solve(39, &Solver039::default());
    solve(40, &Solver040::default());
    solve(41, &Solver041::default());
    solve(42, &Solver042::default());
    solve(43, &Solver043::default());
    solve(44, &Solver044::default());
    solve(45, &Solver045::default());
    solve(46, &Solver046::default());
    solve(47, &Solver047::default());
    solve(48, &Solver048::default());
    solve(49, &Solver049::default());
    solve(50, &Solver050::default());
    solve(51, &Solver051::default());
    solve(52, &Solver052::default());
    solve(53, &Solver053::default());
    solve(54, &Solver054::default());
    solve(55, &Solver055::default());
    solve(56, &Solver056::default());
    solve(57, &Solver057::default());
    solve(58, &Solver058::default());
    solve(59, &Solver059::default());
    solve(60, &Solver060::default());
    solve(61, &Solver061::default());
    solve(62, &Solver062::default());
    solve(63, &Solver063::default());
    solve(64, &Solver064::default());
    solve(65, &Solver065::default());
    solve(66, &Solver066::default());
    solve(67, &Solver067::default());
    solve(68, &Solver068::default());
    solve(69, &Solver069::default());
    solve(70, &Solver070::default());
    solve(71, &Solver071::default());
    solve(72, &Solver072::default());
    solve(73, &Solver073::default());
    solve(74, &Solver074::default());
    solve(75, &Solver075::default());
    solve(76, &Solver076::default());
    solve(77, &Solver077::default());
    solve(78, &Solver078::default());
    solve(79, &Solver079::default());
    solve(80, &Solver080::default());
    solve(81, &Solver081::default());
    solve(82, &Solver082::default());
    solve(83, &Solver083::default());
    solve(84, &Solver084::default());
    solve(85, &Solver085::default());
    solve(86, &Solver086::default());
    solve(87, &Solver087::default());
    solve(88, &Solver088::default());
    solve(89, &Solver089::default());
    solve(90, &Solver090::default());
    solve(91, &Solver091::default());
    solve(92, &Solver092::default());
    solve(93, &Solver093::default());
    solve(94, &Solver094::default());
    solve(95, &Solver095::default());
    solve(96, &Solver096::default());
    solve(97, &Solver097::default());
    solve(98, &Solver098::default());
    solve(99, &Solver099::default());
    solve(100, &Solver100::default());
}

mod euler {
    pub use self::solver001::Solver001;
    pub use self::solver002::Solver002;
    pub use self::solver003::Solver003;
    pub use self::solver004::Solver004;
    pub use self::solver005::Solver005;
    pub use self::solver006::Solver006;
    pub use self::solver007::Solver007;
    pub use self::solver008::Solver008;
    pub use self::solver009::Solver009;
    pub use self::solver010::Solver010;
    pub use self::solver011::Solver011;
    pub use self::solver012::Solver012;
    pub use self::solver013::Solver013;
    pub use self::solver014::Solver014;
    pub use self::solver015::Solver015;
    pub use self::solver016::Solver016;
    pub use self::solver017::Solver017;
    pub use self::solver018::Solver018;
    pub use self::solver019::Solver019;
    pub use self::solver020::Solver020;
    pub use self::solver021::Solver021;
    pub use self::solver022::Solver022;
    pub use self::solver023::Solver023;
    pub use self::solver024::Solver024;
    pub use self::solver025::Solver025;
    pub use self::solver026::Solver026;
    pub use self::solver027::Solver027;
    pub use self::solver028::Solver028;
    pub use self::solver029::Solver029;
    pub use self::solver030::Solver030;
    pub use self::solver031::Solver031;
    pub use self::solver032::Solver032;
    pub use self::solver033::Solver033;
    pub use self::solver034::Solver034;
    pub use self::solver035::Solver035;
    pub use self::solver036::Solver036;
    pub use self::solver037::Solver037;
    pub use self::solver038::Solver038;
    pub use self::solver039::Solver039;
    pub use self::solver040::Solver040;
    pub use self::solver041::Solver041;
    pub use self::solver042::Solver042;
    pub use self::solver043::Solver043;
    pub use self::solver044::Solver044;
    pub use self::solver045::Solver045;
    pub use self::solver046::Solver046;
    pub use self::solver047::Solver047;
    pub use self::solver048::Solver048;
    pub use self::solver049::Solver049;
    pub use self::solver050::Solver050;
    pub use self::solver051::Solver051;
    pub use self::solver052::Solver052;
    pub use self::solver053::Solver053;
    pub use self::solver054::Solver054;
    pub use self::solver055::Solver055;
    pub use self::solver056::Solver056;
    pub use self::solver057::Solver057;
    pub use self::solver058::Solver058;
    pub use self::solver059::Solver059;
    pub use self::solver060::Solver060;
    pub use self::solver061::Solver061;
    pub use self::solver062::Solver062;
    pub use self::solver063::Solver063;
    pub use self::solver064::Solver064;
    pub use self::solver065::Solver065;
    pub use self::solver066::Solver066;
    pub use self::solver067::Solver067;
    pub use self::solver068::Solver068;
    pub use self::solver069::Solver069;
    pub use self::solver070::Solver070;
    pub use self::solver071::Solver071;
    pub use self::solver072::Solver072;
    pub use self::solver073::Solver073;
    pub use self::solver074::Solver074;
    pub use self::solver075::Solver075;
    pub use self::solver076::Solver076;
    pub use self::solver077::Solver077;
    pub use self::solver078::Solver078;
    pub use self::solver079::Solver079;
    pub use self::solver080::Solver080;
    pub use self::solver081::Solver081;
    pub use self::solver082::Solver082;
    pub use self::solver083::Solver083;
    pub use self::solver084::Solver084;
    pub use self::solver085::Solver085;
    pub use self::solver086::Solver086;
    pub use self::solver087::Solver087;
    pub use self::solver088::Solver088;
    pub use self::solver089::Solver089;
    pub use self::solver090::Solver090;
    pub use self::solver091::Solver091;
    pub use self::solver092::Solver092;
    pub use self::solver093::Solver093;
    pub use self::solver094::Solver094;
    pub use self::solver095::Solver095;
    pub use self::solver096::Solver096;
    pub use self::solver097::Solver097;
    pub use self::solver098::Solver098;
    pub use self::solver099::Solver099;
    pub use self::solver100::Solver100;

    pub trait Solver {
        fn solve(&self) -> i64;
    }

    #[cfg(test)]
    mod solver_test;

    #[allow(clippy::module_name_repetitions)]
    pub mod algorithm {
        #[cfg(test)]
        mod algorithm_test;

        pub mod bit;
        pub mod cast;
        pub mod continued_fraction;
        pub mod combinatorics;
        pub mod digits;
        pub mod factor;
        pub mod filter;
        pub mod io;
        pub mod long;
        pub mod prime;
        pub mod root;
        pub mod vec;
    }

    pub mod solver001;
    pub mod solver002;
    pub mod solver003;
    pub mod solver004;
    pub mod solver005;
    pub mod solver006;
    pub mod solver007;
    pub mod solver008;
    pub mod solver009;
    pub mod solver010;
    pub mod solver011;
    pub mod solver012;
    pub mod solver013;
    pub mod solver014;
    pub mod solver015;
    pub mod solver016;
    pub mod solver017;
    pub mod solver018;
    pub mod solver019;
    pub mod solver020;
    pub mod solver021;
    pub mod solver022;
    pub mod solver023;
    pub mod solver024;
    pub mod solver025;
    pub mod solver026;
    pub mod solver027;
    pub mod solver028;
    pub mod solver029;
    pub mod solver030;
    pub mod solver031;
    pub mod solver032;
    pub mod solver033;
    pub mod solver034;
    pub mod solver035;
    pub mod solver036;
    pub mod solver037;
    pub mod solver038;
    pub mod solver039;
    pub mod solver040;
    pub mod solver041;
    pub mod solver042;
    pub mod solver043;
    pub mod solver044;
    pub mod solver045;
    pub mod solver046;
    pub mod solver047;
    pub mod solver048;
    pub mod solver049;
    pub mod solver050;
    pub mod solver051;
    pub mod solver052;
    pub mod solver053;
    pub mod solver054;
    pub mod solver055;
    pub mod solver056;
    pub mod solver057;
    pub mod solver058;
    pub mod solver059;
    pub mod solver060;
    pub mod solver061;
    pub mod solver062;
    pub mod solver063;
    pub mod solver064;
    pub mod solver065;
    pub mod solver066;
    pub mod solver067;
    pub mod solver068;
    pub mod solver069;
    pub mod solver070;
    pub mod solver071;
    pub mod solver072;
    pub mod solver073;
    pub mod solver074;
    pub mod solver075;
    pub mod solver076;
    pub mod solver077;
    pub mod solver078;
    pub mod solver079;
    pub mod solver080;
    pub mod solver081;
    pub mod solver082;
    pub mod solver083;
    pub mod solver084;
    pub mod solver085;
    pub mod solver086;
    pub mod solver087;
    pub mod solver088;
    pub mod solver089;
    pub mod solver090;
    pub mod solver091;
    pub mod solver092;
    pub mod solver093;
    pub mod solver094;
    pub mod solver095;
    pub mod solver096;
    pub mod solver097;
    pub mod solver098;
    pub mod solver099;
    pub mod solver100;
}
