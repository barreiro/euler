/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import static java.util.stream.LongStream.range;
import static net.projecteuler.barreiro.algorithm.util.LongUtils.pow;
import static net.projecteuler.barreiro.algorithm.util.LongUtils.toDigitsStream;

/**
 * Surprisingly there are only three numbers that can be written as the sum of fourth powers of their digits:
 * <p>
 * 1634 = 14 + 64 + 34 + 44
 * 8208 = 84 + 24 + 04 + 84
 * 9474 = 94 + 44 + 74 + 44
 * As 1 = 14 is not a sum it is not included.
 * <p>
 * The sum of these numbers is 1634 + 8208 + 9474 = 19316.
 * <p>
 * Find the sum of all the numbers that can be written as the sum of fifth powers of their digits.
 *
 * @author barreiro
 */
public class Solver030 extends ProjectEulerSolver {

    public Solver030() {
        this( 5 );
    }

    public Solver030(long n) {
        super( n );
    }

    // --- //

    public long solve() {
        return range( N, N * pow( 9, N ) ).filter( n -> n == toDigitsStream( n ).map( d -> pow( d, N ) ).sum() ).sum();
    }

}
