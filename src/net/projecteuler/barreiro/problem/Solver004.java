/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import static java.lang.Math.pow;
import static java.lang.Math.sqrt;
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
        final String candidate = Long.toString(number);
        for (int i = 0; i * 2 < candidate.length(); i++) {
            if (candidate.charAt(i) != candidate.charAt(candidate.length() - i - 1)) return false;
        }
        return true;
    }

    /* --- */

    public long solve() {
        final long roof = (long) pow(10, N); // The max number with N digits
        return rangeReverse(roof * roof, 1).filter(Solver004::isPalindromic).filter(
                // Found a very biggest palindromic number. Try to factor such that the both factors are less than roof
                pal -> rangeReverse(roof, (long) sqrt(pal)).anyMatch(f -> (pal % f == 0) && (pal / f < roof))
        ).findFirst().getAsLong();
    }

}
