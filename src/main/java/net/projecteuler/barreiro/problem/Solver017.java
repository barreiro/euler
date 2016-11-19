/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import static java.util.stream.LongStream.rangeClosed;

/**
 * If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.
 * If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be used?
 * <p/>
 * NOTE: Do not count spaces or hyphens.
 *
 * @author barreiro
 */
public class Solver017 extends ProjectEulerSolver {

    // To better understand where these values come from and the algorithm, check the test for Problem017
    private static final byte[] LOOKUP = {4, 3, 3, 5, 4, 4, 3, 5, 5, 4, 3, 6, 6, 8, 8, 7, 7, 9, 8, 8};
    private static final byte[] LOOKUP10 = {4, 3, 6, 6, 5, 5, 5, 7, 6, 6};

    // Static constants for the length of the commonly used words
    private static final int AND = "and".length();
    private static final int THOUSAND = "thousand".length() + AND;
    private static final int HUNDRED = "hundred".length() + AND;

    public Solver017() {
        this( 1000 );
    }

    public Solver017(long n) {
        super( n );
    }

    // --- //

    private static long letterCount(long number) {
        int k = (int) number, sum = 0;
        if ( k / 1000 > 0 ) {
            sum += LOOKUP[k / 1000] + THOUSAND;
            if ( ( k %= 1000 ) == 0 ) {
                return sum - AND;
            }
        }
        if ( k / 100 > 0 ) {
            sum += LOOKUP[k / 100] + HUNDRED;
            if ( ( k %= 100 ) == 0 ) {
                return sum - AND;
            }
        }
        if ( k / 10 <= 1 ) {
            return sum + LOOKUP[k];
        }
        return sum + LOOKUP10[k / 10] + ( ( k % 10 == 0 ) ? 0 : LOOKUP[k % 10] );
    }

    public long solve() {
        return rangeClosed( 1, N ).map( Solver017::letterCount ).sum();
    }

}
