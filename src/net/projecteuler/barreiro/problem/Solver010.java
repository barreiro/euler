/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import static net.projecteuler.barreiro.algorithm.Primes.primesUpTo;

/**
 * The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
 * Find the sum of all the primes below two million.
 *
 * @author barreiro
 */
public class Solver010 extends ProjectEulerSolver {

    public Solver010() {
        this(2000000);
    }

    public Solver010(long n) {
        super(n);
    }

    /* --- */

    public long solve() {
        return primesUpTo(N).sum();
    }

}
