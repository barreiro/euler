/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.algorithm;

import java.util.PrimitiveIterator.OfLong;
import java.util.function.Predicate;
import java.util.stream.LongStream;

import static net.projecteuler.barreiro.algorithm.util.StreamUtils.lazyStream;

/**
 * Utility methods for Fibonacci numbers
 *
 * @author barreiro
 */
public final class Fibonacci {

    private Fibonacci() {
    }

    /**
     * Creates a stream of fibonacci numbers, starting with 1, 2, ...
     *
     * @param predicate Stop condition
     * @return A stream of fibonacci numbers
     */
    public static LongStream fibonacciStream(Predicate<Long> predicate) {
        return lazyStream( new OfLong() {
            private long previous = 0, last = 1;

            public long nextLong() {
                long result = previous + last;
                previous = last;
                last = result;
                return result;
            }

            public boolean hasNext() {
                return predicate.test( previous + last );
            }
        } );
    }

}
