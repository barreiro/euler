/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

/**
 * The sum of the squares of the first ten natural numbers is, 1^2 + 2^2 + ... + 10^2 = 385
 * The square of the sum of the first ten natural numbers is, (1 + 2 + ... + 10)^2 = 55^2 = 3025
 * Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.
 * <p/>
 * Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
 *
 * @author barreiro
 */
public class Solver006 extends ProjectEulerSolver {

    public Solver006() {
        this(100);
    }

    public Solver006(long n) {
        super(n);
    }

    /* --- */

    public long solve() {
        long squares = 0, sum = (N * (N + 1)) / 2;
        for (long i = 1; i <= N; i++) {
            squares += i * i;
        }
        return sum * sum - squares;
    }

}
