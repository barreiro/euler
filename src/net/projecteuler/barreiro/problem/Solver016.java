/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import java.util.Arrays;

/**
 * 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
 * What is the sum of the digits of the number 2^1000?
 *
 * @author barreiro
 */
public class Solver016 extends ProjectEulerSolver {

    private static final int RADIX = 10;

    public Solver016() {
        this(1000);
    }

    public Solver016(long n) {
        super(n);
    }

    /* --- */

    public long solve() {
        if (N < Long.SIZE - 1) { // if it fits in a long, use the fast method
            return solveL();
        }

        // Each element is a digit. Each iteration we double every digit.
        byte[] values = new byte[]{1};
        for (short i = 0; i < N; i++) {
            // Since carry never ripples we can iterate backwards, using less memory
            for (int j = values.length - 1; j >= 0; j--) {
                values[j] *= 2;
                if (values[j] >= RADIX) {
                    // In first iteration we need to expand the array if there is a carry
                    if (j + 1 == values.length) {
                        values = Arrays.copyOf(values, values.length + 1);
                    }
                    values[j + 1] += values[j] / RADIX;
                    values[j] %= RADIX;
                }
            }
        }
        long sum = 0;
        for (short s : values) {
            sum += s;
        }
        return sum;
    }

    // Anther impl. Works well but just for small N < 63
    public long solveL() {
        long r, q, sum = 0, divisor = 1L << N;
        while (divisor >= RADIX) {
            q = divisor / RADIX;
            r = divisor - (q * RADIX);
            divisor = q;
            sum += r % RADIX;
        }
        return sum + divisor % RADIX;
    }

}
