/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import static net.projecteuler.barreiro.algorithm.Primes.primeFactors;
import static net.projecteuler.barreiro.algorithm.util.StreamUtils.maxLong;

/**
 * The prime factors of 13195 are 5, 7, 13 and 29.
 * What is the largest prime factor of the number 600851475143 ?
 *
 * @author barreiro
 */
public class Solver003 extends ProjectEulerSolver {

    public Solver003() {
        this( 600851475143L );
    }

    public Solver003(long n) {
        super( n );
    }

    // --- //

    public long solve() {
        return maxLong( primeFactors( N ).keySet().stream() );
    }

}
