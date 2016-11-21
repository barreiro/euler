/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import static net.projecteuler.barreiro.algorithm.Primes.millerRabin;
import static net.projecteuler.barreiro.algorithm.Primes.primesLessThan;
import static net.projecteuler.barreiro.algorithm.util.StreamUtils.longList;

/**
 * Euler discovered the remarkable quadratic formula:
 * <p>
 * n^2 + n + 41
 * <p>
 * It turns out that the formula will produce 40 primes for the consecutive values n = 0 to 39.
 * However, when n = 40, 402 + 40 + 41 = 40(40 + 1) + 41 is divisible by 41, and certainly when n = 41, 41^2 + 41 + 41 is clearly divisible by 41.
 * <p>
 * The incredible formula  n^2 − 79n + 1601 was discovered, which produces 80 primes for the consecutive values n = 0 to 79.
 * The product of the coefficients, −79 and 1601, is −126479.
 * <p>
 * Considering quadratics of the form:
 * <p>
 * n^2 + an + b, where |a| < 1000 and |b| < 1000
 * <p>
 * where |n| is the modulus/absolute value of n e.g. |11| = 11 and |−4| = 4
 * <p>
 * Find the product of the coefficients, a and b, for the quadratic expression that produces the maximum number of primes for consecutive values of n, starting with n = 0.
 *
 * @author barreiro
 */
public class Solver027 extends ProjectEulerSolver {

    private static final long HEEGNER = -163;

    public Solver027() {
        this( 1000 );
    }

    public Solver027(long n) {
        super( n );
    }

    // --- //

    public long solve() {
        // Conjecture: a is odd negative and b is one of the 5% highest primes
        // The discriminant must be an Heegner number, in particular -163
        long candidate = 0, best = 0;
        for ( long a = ( N % 2 == 0 ) ? -N + 1 : -N; a < 0; a += 2 ) {
            for ( long b : longList( primesLessThan( N ).limit( N / 20 ) ) ) {
                if ( a * a - 4 * b != HEEGNER ) {
                    continue;
                }

                long n = 0;
                while ( millerRabin( n * n + a * n + b ) ) {
                    n++;
                }
                if ( n > best ) {
                    best = n;
                    candidate = a * b;
                }
            }
        }
        return candidate;
    }

}
