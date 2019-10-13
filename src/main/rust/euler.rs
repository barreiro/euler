// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

pub use euler::*;

fn solve(n: usize, solver: impl euler::Solver) {
    let now = std::time::Instant::now();
    println!("Solution for problem {:03} is {:12} ( took {:9.03} ms )", n, solver.solve(), now.elapsed().as_nanos() as f64 / 1_000_000.0);
}

fn main() {
    solve(1, Solver001::default());
    solve(2, Solver002::default());
    solve(3, Solver003::default());
    solve(4, Solver004::default());
    solve(5, Solver005::default());
    solve(6, Solver006::default());
    solve(7, Solver007::default());
    solve(8, Solver008::default());
    solve(9, Solver009::default());
    solve(10, Solver010::default());
    solve(11, Solver011::default());
    solve(12, Solver012::default());
    solve(13, Solver013::default());
    solve(14, Solver014::default());
    solve(15, Solver015::default());
    solve(16, Solver016::default());
    solve(17, Solver017::default());
    solve(18, Solver018::default());
    solve(19, Solver019::default());
    solve(20, Solver020::default());
    solve(21, Solver021::default());
    solve(22, Solver022::default());
    solve(23, Solver023::default());
    solve(24, Solver024::default());
    solve(25, Solver025::default());
    solve(26, Solver026::default());
    solve(27, Solver027::default());
    solve(28, Solver028::default());
    solve(29, Solver029::default());
    solve(30, Solver030::default());
    solve(31, Solver031::default());
    solve(32, Solver032::default());
    solve(33, Solver033::default());
    solve(34, Solver034::default());
    solve(35, Solver035::default());
    solve(36, Solver036::default());
    solve(37, Solver037::default());
    solve(38, Solver038::default());
    solve(39, Solver039::default());
    solve(40, Solver040::default());
    solve(41, Solver041::default());
    solve(42, Solver042::default());
    solve(43, Solver043::default());
    solve(44, Solver044::default());
    solve(45, Solver045::default());
    solve(46, Solver046::default());
    solve(47, Solver047::default());
    solve(48, Solver048::default());
    solve(49, Solver049::default());
    solve(50, Solver050::default());
    solve(51, Solver051::default());
    solve(52, Solver052::default());
    solve(53, Solver053::default());
    solve(54, Solver054::default());
    solve(55, Solver055::default());
    solve(56, Solver056::default());
    solve(57, Solver057::default());
    solve(58, Solver058::default());
    solve(59, Solver059::default());
    solve(60, Solver060::default());
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

    pub trait Solver {
        fn solve(&self) -> isize;
    }

    #[cfg(test)]
    mod solver_test;

    pub mod algorithm {
        #[cfg(test)]
        mod algorithm_test;

        pub mod bit;
        pub mod combinatorics;
        pub mod factor;
        pub mod long;
        pub mod prime;
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
}
