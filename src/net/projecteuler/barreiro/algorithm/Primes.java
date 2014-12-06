/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.algorithm;

import java.util.*;
import java.util.function.Predicate;
import java.util.stream.LongStream;

import static java.lang.Long.numberOfTrailingZeros;
import static java.util.Arrays.stream;
import static net.projecteuler.barreiro.algorithm.util.LongUtils.powerMod;
import static net.projecteuler.barreiro.algorithm.util.StreamUtils.lazyStream;
import static net.projecteuler.barreiro.algorithm.util.StreamUtils.rangeReverse;

/**
 * Static class with util methods for dealing with prime numbers.
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
        final PrimeGenerator generator = new GeneratorTrialDivision();
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

    /* --- */

    /**
     * Infinite stream of primes.
     *
     * @return A stream of prime numbers
     */
    public static LongStream primesStream() {
        return primesStream(p -> true);
    }

    /**
     * Creates a stream of prime numbers, starting with 2, 3, ... up to N.
     *
     * @param n Upper bound
     * @return A stream of prime numbers
     */
    public static LongStream primesUpTo(final long n) {
        return primesStream(p -> p < n);
    }

    /**
     * Creates a stream of prime numbers, starting with the one below N.
     *
     * @param n Upper bound
     * @return A stream of prime numbers
     */
    public static LongStream primesLessThan(final long n) {
        return rangeReverse(n, 1).filter(Primes::millerRabin);
    }

    /**
     * Infinite stream of primes, using Miller-Rabin prime test.
     *
     * @return A stream of prime numbers
     */
    public static LongStream primesStreamMillerRabin() {
        return primesStream(p -> true, new GeneratorMillerRabin());
    }

    private static LongStream primesStream(final Predicate<Long> predicate) {
        return primesStream(predicate, new GeneratorTrialDivision());
    }

    private static LongStream primesStream(final Predicate<Long> predicate, PrimeGenerator generator) {
        return lazyStream(new PrimitiveIterator.OfLong() {

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

    /* --- */

    /**
     * Base of values to use in Miller-Rabin test. Accurate up to 2^64.
     */
    private static long[] MILLER_RABIN_BASE = new long[]{2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47};

    /**
     * Prime test using Miller-Rabin test.
     *
     * @param n Value to test
     * @return true if the number is prime
     */
    public static boolean millerRabin(long n) {
        return (n > 1) && ((n == 2) || stream(MILLER_RABIN_BASE).allMatch(b -> n <= b || millerRabinPass(b, n)));
    }

    private static boolean millerRabinPass(final long a, final long n) {
        long y = powerMod(a, n - 1, n);
        if ((y == 1) || (y == n - 1)) return true;
        for (long l = 0; l < numberOfTrailingZeros(n - 1) - 1; l++) {
            y = powerMod(y, 2, n);
            if (y == n - 1) return true;
        }
        return y == n - 1;
    }

    /* --- */

    private static interface PrimeGenerator {
        public long nextPrime();
    }

    /**
     * Helper class to generate Prime numbers using trial division.
     */
    private static class GeneratorTrialDivision implements PrimeGenerator {

        private Deque<Long> primeCache = null;

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

    /**
     * Helper class to generate Prime numbers using Miller-Rabin test. For small number it's slower than trial division.
     */
    private static class GeneratorMillerRabin implements PrimeGenerator {

        private long last = 0;

        public long nextPrime() {
            if (last == 0) return ++last + 1;
            last += 2;
            while (!millerRabin(last)) {
                last += 2;
            }
            return last;
        }
    }

}
