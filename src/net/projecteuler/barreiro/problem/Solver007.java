/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import static net.projecteuler.barreiro.algorithm.Primes.primesStream;

/**
 * By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
 * What is the 10001st prime number?
 *
 * @author barreiro
 */
public class Solver007 extends ProjectEulerSolver {

    public Solver007() {
        this(10001);
    }

    public Solver007(long n) {
        super(n);
    }

    /* --- */

    public long solve() {
        return primesStream().limit(N).max().getAsLong();
    }

}
