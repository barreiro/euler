/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import static java.util.Arrays.stream;
import static java.util.stream.LongStream.range;

/**
 * If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
 * Find the sum of all the multiples of 3 or 5 below 1000.
 *
 * @author barreiro
 */
public class Solver001 extends ProjectEulerSolver {

    private final long[] FACTORS;

    public Solver001() {
        this(1000);
    }

    public Solver001(long n) {
        super(n);
        this.FACTORS = new long[]{3, 5};
    }

    /* --- */

    public long solve() {
        return range(0, N).filter(l -> stream(FACTORS).anyMatch(f -> l % f == 0)).sum();
    }

}
