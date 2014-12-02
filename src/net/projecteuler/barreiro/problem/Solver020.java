/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import java.util.LinkedList;

import static java.util.Arrays.stream;
import static java.util.stream.IntStream.range;
import static java.util.stream.LongStream.rangeClosed;

/**
 * n! means n × (n − 1) × ... × 3 × 2 × 1
 * <p>
 * For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800, and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.
 * <p>
 * Find the sum of the digits in the number 100!
 *
 * @author barreiro
 */
public class Solver020 extends ProjectEulerSolver {

    private static final int RADIX = 10;

    public Solver020() {
        this(100);
    }

    public Solver020(long n) {
        super(n);
    }

    /* --- */

    public long solve() {
        if (N <= 2) return N;
        return stream(rangeClosed(2, N).mapToObj(Solver020::toDigits).reduce(Solver020::multiplication).get()).sum();
    }

    /* --- */

    // Both numbers need to be arrays, or else there is the risk of carry overflow
    private static long[] multiplication(long[] a, long[] b) {
        long[] result = new long[a.length + b.length];
        range(0, a.length).forEach(i -> range(0, b.length).forEach(j -> result[i + j] += a[i] * b[j]));

        for (int carry = 0, i = 0; i < result.length; i++) {
            result[i] += carry;
            carry = (int) result[i] / RADIX;
            result[i] -= carry * RADIX;
        }
        return result;
    }

    private static long[] toDigits(long number) {
        LinkedList<Long> digits = new LinkedList<>();
        for (; number >= RADIX; number /= RADIX) {
            digits.addFirst(number % RADIX);
        }
        digits.addFirst(number);
        return digits.stream().mapToLong(l -> l).toArray();
    }
}
