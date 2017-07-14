/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.algorithm;

import static java.util.stream.LongStream.rangeClosed;
import static net.projecteuler.barreiro.algorithm.util.LongUtils.intSqrt;
import static net.projecteuler.barreiro.algorithm.util.StreamUtils.rangeReverse;

/**
 * Utility methods for dealing with the always hard problem of factorization.
 * It is wise to use parallel streams when factorization is involved.
 *
 * @author barreiro
 */
public final class Factorization {

    private Factorization() {
    }

    /**
     * Finds if a value has two factors below a certain roof.
     *
     * @param value Value to factorize
     * @param roof  Max value of the factors
     * @return true if there are 2 factors that meet the condition
     */
    public static boolean hasFactorsBelow(long value, long roof) {
        return rangeReverse( roof, intSqrt( value ) ).anyMatch( f -> ( value % f == 0 ) && ( value / f < roof ) );
    }

    /**
     * Calculates the number of factors of a number.
     *
     * @return the number of factors
     */
    public static long numberOfFactors(long value) {
        // We need to adjust the number of divisors if the number is a perfect square
        long ceiling = intSqrt( value );
        long divisors = ( ceiling * ceiling == value ) ? -1 : 0;
        return divisors + rangeClosed( 1, ceiling ).filter( f -> value % f == 0 ).count() * 2;
    }

    /**
     * Calculates the sum of the factors of a number.
     *
     * @return the sum of factors
     */
    public static long sumFactors(long value) {
        // We need to adjust the number of divisors if the number is a perfect square
        long ceiling = intSqrt( value );
        long squareFactor = ( ceiling * ceiling == value ) ? -ceiling : 0;
        return 1 + squareFactor + rangeClosed( 2, ceiling ).filter( f -> value % f == 0 ).map( f -> f + value / f ).sum();
    }

    /**
     * A number is said to be abundant if it's smaller than the sum of its factors.
     *
     * @return true if the number is abundant
     */
    public static boolean isAbundant(long value) {
        return value < sumFactors( value );
    }
}
