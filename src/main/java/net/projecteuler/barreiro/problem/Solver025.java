/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import static java.util.Arrays.copyOf;
import static net.projecteuler.barreiro.algorithm.util.LongUtils.addition;
import static net.projecteuler.barreiro.algorithm.util.LongUtils.toDigits;

/**
 * The Fibonacci sequence is defined by the recurrence relation:
 * <p>
 * Fn = Fn−1 + Fn−2, where F1 = 1 and F2 = 1.
 * <p>
 * Hence the first 12 terms will be:
 * F1 = 1
 * F2 = 1
 * F3 = 2
 * F4 = 3
 * F5 = 5
 * F6 = 8
 * F7 = 13
 * F8 = 21
 * F9 = 34
 * F10 = 55
 * F11 = 89
 * F12 = 144
 * The 12th term, F12, is the first term to contain three digits.
 * <p>
 * What is the first term in the Fibonacci sequence to contain 1000 digits?
 *
 * @author barreiro
 */
public class Solver025 extends ProjectEulerSolver {

    public Solver025() {
        this( 1000 );
    }

    public Solver025(long n) {
        super( n );
    }

    // --- //

    public long solve() {
        long[] previous = new long[(int) N];
        long[] last = copyOf( toDigits( 1 ), (int) N );
        long[] current = copyOf( toDigits( 1 ), (int) N );

        for ( long t = 3; ; t++ ) {
            long[] next = previous;
            previous = last;
            last = current;
            current = addition( previous, last, next );
            if ( current[(int) N - 1] != 0 ) {
                return t;
            }
        }
    }

}
