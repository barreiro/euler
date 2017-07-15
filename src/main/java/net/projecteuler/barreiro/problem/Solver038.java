/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import net.projecteuler.barreiro.algorithm.util.LongUtils;

import static java.util.stream.LongStream.rangeClosed;
import static net.projecteuler.barreiro.algorithm.util.LongUtils.concatenation;
import static net.projecteuler.barreiro.algorithm.util.LongUtils.intSqrt;
import static net.projecteuler.barreiro.algorithm.util.LongUtils.isPandigital;
import static net.projecteuler.barreiro.algorithm.util.LongUtils.pow10;
import static net.projecteuler.barreiro.algorithm.util.LongUtils.toDigits;
import static net.projecteuler.barreiro.algorithm.util.StreamUtils.maxLong;

/**
 * Take the number 192 and multiply it by each of 1, 2, and 3:
 * 192 × 1 = 192
 * 192 × 2 = 384
 * 192 × 3 = 576
 * By concatenating each product we get the 1 to 9 pandigital, 192384576. We will call 192384576 the concatenated product of 192 and (1,2,3)
 * <p>
 * The same can be achieved by starting with 9 and multiplying by 1, 2, 3, 4, and 5, giving the pandigital, 918273645, which is the concatenated product of 9 and (1,2,3,4,5).
 * <p>
 * What is the largest 1 to 9 pandigital 9-digit number that can be formed as the concatenated product of an integer with (1,2, ... , n) where n > 1?
 *
 * @author barreiro
 */
public class Solver038 extends ProjectEulerSolver {

    public Solver038() {
        this( 9 );
    }

    public Solver038(long n) {
        super( n );
    }

    // --- //

    // generate the natural product concatenation from every multiplier (within a sensible range) and check if it's a pandigital
    public long solve() {
        return maxLong( rangeClosed( intSqrt( N ), pow10( intSqrt( N ) + 1 ) ).mapToObj( n -> {
            long value = n;
            long[] digits = toDigits( value );
            while ( digits.length < N ) {
                value += n;
                digits = concatenation( digits, toDigits( value ) );
            }
            return digits;
        } ).filter( d -> d.length == N && isPandigital( d ) ).mapToLong( LongUtils::fromDigits ) );
    }
}
