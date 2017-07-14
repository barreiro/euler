/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;

import static java.util.stream.LongStream.range;
import static net.projecteuler.barreiro.algorithm.Factorization.sumFactors;

/**
 * Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide evenly into n).
 * If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and each of a and b are called amicable numbers.
 * <p>
 * For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110; therefore d(220) = 284. The proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.
 * <p>
 * Evaluate the sum of all the amicable numbers under 10000.
 *
 * @author barreiro
 */
public class Solver021 extends ProjectEulerSolver {

    public Solver021() {
        this( 10000 );
    }

    public Solver021(long n) {
        super( n );
    }

    // --- //

    private boolean isAmicable(long value, Map<Long, Long> factorSum) {
        long sum = factorSum.get( value );
        return sum > 1 && sum < N && sum != value && factorSum.get( sum ) == value;
    }

    public long solve() {
        Map<Long, Long> factorSum = new ConcurrentHashMap<>();
        range( 2, N ).parallel().forEach( l -> factorSum.put( l, sumFactors( l ) ) );
        return range( 2, N ).filter( l -> isAmicable( l, factorSum ) ).sum();
    }

}
