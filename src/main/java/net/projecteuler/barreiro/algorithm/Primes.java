/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.algorithm;

import java.util.ArrayDeque;
import java.util.Deque;
import java.util.HashMap;
import java.util.Map;
import java.util.PrimitiveIterator.OfLong;
import java.util.function.Predicate;
import java.util.stream.LongStream;

import static java.lang.Long.numberOfTrailingZeros;
import static java.util.Arrays.stream;
import static net.projecteuler.barreiro.algorithm.util.LongUtils.fromDigits;
import static net.projecteuler.barreiro.algorithm.util.LongUtils.powerMod;
import static net.projecteuler.barreiro.algorithm.util.LongUtils.toDigitsStream;
import static net.projecteuler.barreiro.algorithm.util.StreamUtils.lazyStream;
import static net.projecteuler.barreiro.algorithm.util.StreamUtils.rangeReverse;

/**
 * Utility methods for dealing with prime numbers.
 *
 * @author barreiro
 */
public final class Primes {

    /**
     * Base of values to use in Miller-Rabin test. Accurate up to 2^32 and 2^64.
     */
    private static final long[] MILLER_RABIN_FAST = new long[]{2, 7, 61};
    private static final long[] MILLER_RABIN_BASE = new long[]{2, 325, 9375, 28178, 450775, 9780504, 1795265022};

    private Primes() {
    }

    // --- //

    /**
     * Calculates the prime factors.
     *
     * @param l The number to calculate the factors
     * @return A map containing the factor and the power
     */
    public static Map<Long, Long> primeFactors(long l) {
        Map<Long, Long> factorMap = new HashMap<>();
        PrimeGenerator generator = new GeneratorTrialDivision();
        long factor;
        long subject = l;
        do {
            factor = generator.nextPrime();
            long count = 0;
            while ( subject % factor == 0 ) {
                subject /= factor;
                count++;
            }
            if ( count != 0 ) {
                factorMap.put( factor, count );
            }
            if ( subject == 1 ) {
                break;
            }
        } while ( factor * factor < l );
        if ( ( subject >= factor ) || ( factorMap.isEmpty() ) ) {
            // If number is prime add itself
            factorMap.put( subject, 1L );
        }
        return factorMap;
    }

    /**
     * Test if a number is made from prime digits
     *
     * @return False if any of the digits not not allow all permutations to be prime
     */
    public static boolean testPrimePermutation(long n) {
        return n < 9 || toDigitsStream( n ).allMatch( l -> l == 1 || l == 3 || l == 7 || l == 9 );
    }

    /**
     * Infinite stream of primes.
     *
     * @return A stream of prime numbers
     */
    public static LongStream primesStream() {
        return primesStream( p -> true );
    }

    /**
     * Creates a stream of prime numbers, starting with 2, 3, ... up to N.
     *
     * @return A stream of prime numbers
     */
    public static LongStream primesStream(long n) {
        return primesStream( p -> p < n );
    }

    /**
     * Creates a stream of prime numbers, starting with the one below N.
     *
     * @return A stream of prime numbers
     */
    public static LongStream primesLessThan(long n) {
        return rangeReverse( n, 1 ).filter( Primes::millerRabin );
    }

    /**
     * Infinite stream of primes, using Miller-Rabin prime test.
     *
     * @return A stream of prime numbers
     */
    public static LongStream primesStreamMillerRabin() {
        return primesStream( p -> true, new GeneratorMillerRabin() );
    }

    // --- //

    private static LongStream primesStream(Predicate<Long> predicate) {
        return primesStream( predicate, new GeneratorTrialDivision() );
    }

    private static LongStream primesStream(Predicate<Long> predicate, PrimeGenerator generator) {
        return lazyStream( new OfLong() {

            private long next = generator.nextPrime();

            public long nextLong() {
                long last = next;
                next = generator.nextPrime();
                return last;
            }

            public boolean hasNext() {
                return predicate.test( next );
            }
        } );
    }

    /**
     * Prime test using Miller-Rabin test.
     *
     * @return true if the number is prime
     */
    public static boolean millerRabin(long... n) {
        return millerRabin( fromDigits( n ) );
    }

    /**
     * Prime test using Miller-Rabin test.
     *
     * @return true if the number is prime
     */
    public static boolean millerRabin(long n) {
        long[] effectiveBase = n < 4759123141L ? MILLER_RABIN_FAST : MILLER_RABIN_BASE;
        return ( n > 1 ) && ( ( n == 2 ) || stream( effectiveBase ).allMatch( b -> n <= b || millerRabinPass( b, n ) ) );
    }

    private static boolean millerRabinPass(long b, long n) {
        long s = numberOfTrailingZeros( n - 1 );
        long d = ( n - 1 ) >> s;
        long a = powerMod( b, d, n );
        if ( a == 1 ) {
            return true;
        }
        for ( long i = 0; i < s - 1; i++ ) {
            if ( a == n - 1 ) {
                return true;
            }
            if ( a == 1 ) {
                return false;
            }
            a = powerMod( a, 2, n );
        }
        return a == n - 1;
    }

    // --- //

    private interface PrimeGenerator {
        long nextPrime();
    }

    /**
     * Helper class to generate Prime numbers using trial division.
     */
    private static class GeneratorTrialDivision implements PrimeGenerator {

        private Deque<Long> primeCache = null;

        public long nextPrime() {
            // Avoid put 2 into the cache. It's easy to skip all even numbers
            if ( primeCache == null ) {
                primeCache = new ArrayDeque<>();
                return 2;
            }
            if ( primeCache.isEmpty() ) {
                primeCache.add( 3L );
                return 3;
            }
            for ( long candidate = primeCache.getLast() + 2; ; candidate += 2 ) {
                for ( long prime : primeCache ) {
                    if ( candidate % prime == 0 ) {
                        break;
                    }

                    if ( prime * prime > primeCache.getLast() ) {
                        primeCache.add( candidate );
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
            if ( last == 0 ) {
                return ++last + 1;
            }
            last += 2;
            while ( !millerRabin( last ) ) {
                last += 2;
            }
            return last;
        }
    }

}
