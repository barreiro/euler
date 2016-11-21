/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import net.projecteuler.barreiro.algorithm.Primes;

import static net.projecteuler.barreiro.algorithm.Combinatorics.rotationStream;
import static net.projecteuler.barreiro.algorithm.Primes.primesStream;
import static net.projecteuler.barreiro.algorithm.util.LongUtils.toDigitsStream;

/**
 * The number, 197, is called a circular prime because all rotations of the digits: 197, 971, and 719, are themselves prime.
 * <p>
 * There are thirteen such primes below 100: 2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, and 97.
 * <p>
 * How many circular primes are there below one million?
 *
 * @author barreiro
 */
public class Solver035 extends ProjectEulerSolver {

    public Solver035() {
        this( 1000000 );
    }

    public Solver035(long n) {
        super( n );
    }

    // --- //

    // Optimization so that the rotation stream is not created. If there are even digits in a prime there are rotations that will not be prime.
    private static boolean oddCheck(long p) {
        return p <= 5 || toDigitsStream( p ).allMatch( i -> i == 1 || i == 3 || i == 7 | i == 9 );
    }

    public long solve() {
        return primesStream( N ).filter( Solver035::oddCheck ).filter( p -> rotationStream( p ).allMatch( Primes::millerRabin ) ).count();
    }

}
