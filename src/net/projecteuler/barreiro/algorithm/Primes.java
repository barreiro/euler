/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.algorithm;

import java.util.*;
import java.util.function.Predicate;
import java.util.stream.LongStream;

import static net.projecteuler.barreiro.algorithm.util.StreamUtils.lazyStream;

/**
 * Static class with util methods for solving the problems.
 *
 * @author barreiro
 */
public final class Primes {

    private Primes() {
    }

    /**
     * Calculates the prime factors.
     *
     * @param subject The number to calculate the factors
     * @return A map containing the factor and the power
     */
    public static Map<Long, Long> primeFactors(long subject) {
        final Map<Long, Long> factorMap = new HashMap<>();
        final Generator generator = new Generator();
        long factor, ceiling = subject;
        do {
            factor = generator.nextPrime();
            long count = 0;
            while (subject % factor == 0) {
                subject /= factor;
                count++;
            }
            if (count != 0) factorMap.put(factor, count);
            if (subject == 1) break;
        } while (factor * factor < ceiling);
        if (factorMap.isEmpty()) factorMap.put(subject, 1L); // If number is prime add itself
        return factorMap;
    }

    /**
     * Infinite stream of primes.
     *
     * @return A stream of prime numbers
     */
    public static LongStream primesStream() {
        return primesStream(p -> true);
    }

    /**
     * Creates a stream of prime numbers, starting with 2, 3, ...
     *
     * @param predicate Stop condition
     * @return A stream of prime numbers
     */
    public static LongStream primesStream(final Predicate<Long> predicate) {
        return lazyStream(new PrimitiveIterator.OfLong() {
            private final Generator generator = new Generator();
            private long next = generator.nextPrime();

            public long nextLong() {
                long last = next;
                next = generator.nextPrime();
                return last;
            }

            public boolean hasNext() {
                return predicate.test(next);
            }
        });
    }

    /**
     * Helper class to generate Prime numbers.
     */
    public static class Generator {

        private Deque<Long> primeCache = null;

        /**
         * Get the next one.
         *
         * @return The next prime from this Prime generator.
         */
        public long nextPrime() {
            // Avoid put 2 into the cache. It's easy to skip all even numbers
            if (primeCache == null) {
                primeCache = new LinkedList<>();
                return 2;
            }
            if (primeCache.isEmpty()) {
                primeCache.add(3L);
                return 3;
            }
            final long last = primeCache.getLast();
            for (long candidate = last % 2 == 0 ? last + 1 : last + 2; ; candidate += 2) {
                for (long prime : primeCache) {
                    if (candidate % prime == 0) break;

                    if (prime * prime > last) {
                        primeCache.add(candidate);
                        return candidate;
                    }
                }
            }
        }
    }

}
