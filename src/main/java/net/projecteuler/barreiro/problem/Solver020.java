/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import net.projecteuler.barreiro.algorithm.util.LongUtils;

import static java.util.Arrays.stream;
import static net.projecteuler.barreiro.algorithm.Combinatorics.digitStream;

/**
 * n! means n × (n − 1) × ... × 3 × 2 × 1
 * <p>
 * For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800, and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.
 * <p>
 * Find the sum of the digits in the number 100!
 *
 * @author barreiro
 */
public class Solver020 extends ProjectEulerSolver {

    private static final long[] EMPTY = new long[]{};

    public Solver020() {
        this( 100 );
    }

    public Solver020(long n) {
        super( n );
    }

    // --- //

    public long solve() {
        return stream( digitStream( 1, N ).reduce( LongUtils::multiplication ).orElse( EMPTY ) ).sum();
    }

}
