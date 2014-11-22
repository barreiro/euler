/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

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
        long sum = 0;
        for (long i = 1; i < N; i++) {
            for (long factor : FACTORS) {
                if (i % factor == 0) {
                    sum += i;
                    break;
                }
            }
        }
        return sum;
    }

}
