/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import java.util.function.LongPredicate;

import static net.projecteuler.barreiro.algorithm.Primes.millerRabin;
import static net.projecteuler.barreiro.algorithm.Primes.primesStream;

/**
 * The number 3797 has an interesting property. Being prime itself, it is possible to continuously remove digits from left to right, and remain prime at each stage: 3797, 797, 97, and 7. Similarly we can work from right to left: 3797, 379, 37, and 3.
 * <p>
 * Find the sum of the only eleven primes that are both truncatable from left to right and right to left.
 * <p>
 * NOTE: 2, 3, 5, and 7 are not considered to be truncatable primes.
 *
 * @author barreiro
 */
public class Solver037 extends ProjectEulerSolver {

    private static final LongPredicate TRUNCATABLE_RIGHT = p -> {
        for ( long l = p / 10; l > 0; l /= 10 ) {
            if ( !millerRabin( l ) ) {
                return false;
            }
        }
        return true;
    };

    private static final LongPredicate TRUNCATABLE_LEFT = p -> {
        for ( long l, n = 10; ( l = p % n ) < p; n *= 10 ) {
            if ( !millerRabin( l ) ) {
                return false;
            }
        }
        return true;
    };

    public Solver037() {
        this( 11 );
    }

    public Solver037(long n) {
        super( n );
    }

    // --- //

    public long solve() {
        return primesStream().filter( TRUNCATABLE_LEFT.and( TRUNCATABLE_RIGHT ) ).filter( p -> p > 9 ).limit( N ).sum();
    }

}
