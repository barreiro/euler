/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import net.projecteuler.barreiro.algorithm.util.LongUtils;

import static net.projecteuler.barreiro.algorithm.Factorization.hasFactorsBelow;
import static net.projecteuler.barreiro.algorithm.util.LongUtils.pow;
import static net.projecteuler.barreiro.algorithm.util.LongUtils.pow10;
import static net.projecteuler.barreiro.algorithm.util.StreamUtils.firstLong;
import static net.projecteuler.barreiro.algorithm.util.StreamUtils.rangeReverse;

/**
 * A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
 * Find the largest palindrome made from the product of two 3-digit numbers.
 *
 * @author barreiro
 */
public class Solver004 extends ProjectEulerSolver {

    public Solver004() {
        this( 3 );
    }

    public Solver004(long n) {
        super( n );
    }

    // --- //

    public long solve() {
        return firstLong( rangeReverse( pow( pow10( N ), 2 ), 1 ).filter( LongUtils::isPalindrome ).filter( pal -> hasFactorsBelow( pal, pow10( N ) ) ) );
    }

}
