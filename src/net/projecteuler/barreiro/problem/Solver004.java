/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import static net.projecteuler.barreiro.algorithm.Factorization.hasFactorsBelow;
import static net.projecteuler.barreiro.algorithm.util.LongUtils.pow;
import static net.projecteuler.barreiro.algorithm.util.LongUtils.toDigits;
import static net.projecteuler.barreiro.algorithm.util.StreamUtils.rangeReverse;

/**
 * A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
 * Find the largest palindrome made from the product of two 3-digit numbers.
 *
 * @author barreiro
 */
public class Solver004 extends ProjectEulerSolver {

    public Solver004() {
        this(3);
    }

    public Solver004(long n) {
        super(n);
    }

    /* --- */

    public static boolean isPalindromic(long number) {
        final long[] digits = toDigits(number);
        for (int i = 0; i * 2 < digits.length; i++) {
            if (digits[i] != digits[digits.length - i - 1]) return false;
        }
        return true;
    }

    /* --- */

    public long solve() {
        final long roof = pow(10, N); // The max number with N digits
        return rangeReverse(roof * roof, 1).filter(Solver004::isPalindromic).filter(pal -> hasFactorsBelow(pal, roof)).findFirst().getAsLong();
    }

}
