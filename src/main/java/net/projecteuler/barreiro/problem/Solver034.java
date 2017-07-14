/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import static java.util.Arrays.stream;
import static net.projecteuler.barreiro.algorithm.Combinatorics.digitStream;
import static net.projecteuler.barreiro.algorithm.util.LongUtils.factorial;
import static net.projecteuler.barreiro.algorithm.util.LongUtils.fromDigits;
import static net.projecteuler.barreiro.algorithm.util.StreamUtils.digitSum;

/**
 * 145 is a curious number, as 1! + 4! + 5! = 1 + 24 + 120 = 145.
 * <p>
 * Find the sum of all numbers which are equal to the sum of the factorial of their digits.
 * <p>
 * Note: as 1! = 1 and 2! = 2 are not sums they are not included.
 *
 * @author barreiro
 */
public class Solver034 extends ProjectEulerSolver {

    private static final long[] FACTORIAL_CACHE = new long[]{1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880};

    public Solver034() {
        this( 2 * factorial( 9 ) );
    }

    public Solver034(long n) {
        super( n );
    }

    // --- //

    private long fastFactorialSum(long... l) {
        return stream( l ).map( d -> FACTORIAL_CACHE[(int) d] ).sum();
    }

    public long solve() {
        return digitSum( digitStream( 3, N ).parallel().filter( d -> fromDigits( d ) == fastFactorialSum( d ) ) );
    }

}
