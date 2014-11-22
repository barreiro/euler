/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

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

    protected static boolean isPalindromic(long number) {
        String candidate = Long.toString(number);
        for (int i = 0; i * 2 < candidate.length(); i++) {
            if (candidate.charAt(i) != candidate.charAt(candidate.length() - i - 1)) {
                return false;
            }
        }
        return true;
    }

    /* --- */

    public long solve() {
        long roof = (long) Math.pow(10, N);
        for (long l = roof * roof; l > 1; l--) {
            if (isPalindromic(l)) { // Found a very biggest palindromic number. Try to factor.
                for (long candidate = roof; candidate >= Math.sqrt(l); candidate--) {
                    if ((l % candidate == 0) && (l / candidate < roof)) {
                        return l;
                    }
                }
            }
        }
        return 0;
    }

}
