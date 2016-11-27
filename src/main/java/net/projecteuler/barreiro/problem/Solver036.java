/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import static net.projecteuler.barreiro.algorithm.Combinatorics.palindromeStream;
import static net.projecteuler.barreiro.algorithm.util.LongUtils.isPalindrome;

/**
 * The decimal number, 585 = 10010010012 (binary), is palindromic in both bases.
 * <p>
 * Find the sum of all numbers, less than one million, which are palindromic in base 10 and base 2.
 * <p>
 * (Please note that the palindromic number, in either base, may not include leading zeros.)
 *
 * @author barreiro
 */
public class Solver036 extends ProjectEulerSolver {

    public Solver036() {
        this( 1000000 );
    }

    public Solver036(long n) {
        super( n );
    }

    // --- //

    public long solve() {
        return palindromeStream( N ).filter( p -> isPalindrome( p, 2 ) ).sum();
    }

}
