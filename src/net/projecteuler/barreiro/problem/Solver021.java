/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import java.util.HashMap;
import java.util.Map;

import static java.lang.Math.sqrt;
import static java.util.stream.LongStream.range;
import static java.util.stream.LongStream.rangeClosed;

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
        this(10000);
    }

    public Solver021(long n) {
        super(n);
    }

    /* --- */

    public long solve() {
        Map<Long, Long> factorSum = new HashMap<>();
        range(2, N).parallel().forEach(l -> factorSum.put(l, sumFactors(l)));
        return range(2, N).filter(l -> {
            long s = factorSum.get(l);
            return (s > 1 && s < N) && (s != l && factorSum.get(s) == l);
        }).sum();
    }

    private static long sumFactors(long number) {
        // We need to adjust the number of divisors if the number is a perfect square
        long ceiling = (long) sqrt(number), squareFactor = (ceiling * ceiling == number) ? -ceiling : 0;
        return 1 + squareFactor + rangeClosed(2, ceiling).filter(f -> number % f == 0).map(f -> f + number / f).sum();
    }

}
