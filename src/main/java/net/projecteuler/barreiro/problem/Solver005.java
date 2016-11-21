/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import net.projecteuler.barreiro.algorithm.Primes;

import java.util.HashMap;
import java.util.Map;

import static java.util.stream.LongStream.rangeClosed;
import static net.projecteuler.barreiro.algorithm.util.LongUtils.pow;
import static net.projecteuler.barreiro.algorithm.util.StreamUtils.product;

/**
 * 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
 * What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
 *
 * @author barreiro
 */
public class Solver005 extends ProjectEulerSolver {

    public Solver005() {
        this( 20 );
    }

    public Solver005(long n) {
        super( n );
    }

    // --- //

    public long solve() {
        Map<Long, Long> factorMap = new HashMap<>();

        // Populate the factorMap with the highest values of each of the factors
        rangeClosed( 1, N ).mapToObj( Primes::primeFactors ).forEach( fm -> fm.forEach( (k, v) -> factorMap.merge( k, v, Long::max ) ) );

        // Calculate the product of the factors
        return product( factorMap.entrySet().stream().mapToLong( e -> pow( e.getKey(), e.getValue() ) ) );
    }

}
