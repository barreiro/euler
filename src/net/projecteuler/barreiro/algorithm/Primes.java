/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.algorithm;

import java.util.*;

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
     * @param primes  A set of numbers to be used as a sieve
     * @return A map containing the factor and the power
     */
    public static Map<Long, Long> primeFactors(long subject, Set<Long> primes) {
        Map<Long, Long> factorMap = new HashMap<>();
        for (long factor : primes) {
            long count = 0;
            while (subject % factor == 0) {
                subject /= factor;
                count++;
            }
            if (count != 0) {
                factorMap.put(factor, count);
            }
            if (subject == 1) {
                break;
            }
        }
        return factorMap;
    }

    /**
     * Helper class to generate Prime numbers.
     */
    public static class Generator {

        private SortedSet<Long> primeCache;

        /**
         * Get the next one.
         *
         * @return The next prime from this Prime generator.
         */
        public long nextPrime() {
            if (primeCache == null) { // Trick to avoid putting 2 into the cache, making possible to skip even numbers
                primeCache = new TreeSet<>();
                return 2L;
            }
            if (primeCache.isEmpty()) {
                primeCache.add(3L);
                return 3l;
            }
            long last = primeCache.last(), lowerBound = (long) Math.sqrt(last);
            for (long candidate = last % 2 == 0 ? last + 1 : last + 2; ; candidate += 2) {
                for (long prime : primeCache) {
                    if (candidate % prime == 0) {
                        break;
                    }
                    if (prime > lowerBound) {
                        primeCache.add(candidate);
                        return candidate;
                    }
                }
            }
        }
    }

}
