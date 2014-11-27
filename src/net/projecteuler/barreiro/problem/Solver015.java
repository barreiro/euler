/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import static net.projecteuler.barreiro.algorithm.Combinatorics.choose;

/**
 * Starting in the top left corner of a 2x2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.
 * How many such routes are there through a 20x20 grid?
 *
 * @author barreiro
 */
public class Solver015 extends ProjectEulerSolver {

    public Solver015() {
        this(20);
    }

    public Solver015(long n) {
        super(n);
    }

    /* --- */

    public long solve() {
        return choose(2 * N, N);
    }

}
