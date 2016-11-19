/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import static java.util.Arrays.copyOf;
import static java.util.Arrays.stream;

/**
 * 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
 * What is the sum of the digits of the number 2^1000?
 *
 * @author barreiro
 */
public class Solver016 extends ProjectEulerSolver {

    private static final int RADIX = 10;

    public Solver016() {
        this( 1000 );
    }

    public Solver016(long n) {
        super( n );
    }

    // --- //

    public long solve() {
        // Each element is a digit. Each iteration we double every digit and adjust.
        int[] values = new int[]{1};

        for ( int i = 0; i < N; i++ ) {
            // Since carry never ripples we can iterate backwards, using less memory
            for ( int j = values.length - 1; j >= 0; j-- ) {
                values[j] *= 2;
                if ( values[j] >= RADIX ) {
                    // In first iteration we may need to expand the array
                    if ( j + 1 == values.length ) {
                        values = copyOf( values, values.length + 1 );
                    }

                    values[j + 1] += values[j] / RADIX;
                    values[j] %= RADIX;
                }
            }
        }
        return stream( values ).sum();
    }

}
