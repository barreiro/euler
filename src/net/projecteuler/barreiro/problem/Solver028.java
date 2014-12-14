/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import net.projecteuler.barreiro.algorithm.util.LongUtils;

import static java.util.stream.LongStream.rangeClosed;
import static net.projecteuler.barreiro.algorithm.util.LongUtils.pow;

/**
 * Starting with the number 1 and moving to the right in a clockwise direction a 5 by 5 spiral is formed as follows:
 * <p>
 * 21 22 23 24 25
 * 20  7  8  9 10
 * 19  6  1  2 11
 * 18  5  4  3 12
 * 17 16 15 14 13
 * <p>
 * It can be verified that the sum of the numbers on the diagonals is 101.
 * <p>
 * What is the sum of the numbers on the diagonals in a 1001 by 1001 spiral formed in the same way?
 *
 * @author barreiro
 */
public class Solver028 extends ProjectEulerSolver {

    public Solver028() {
        this(1001);
    }

    public Solver028(long n) {
        super(n);
    }

    /* --- */

    public long solve() {
        return 1 + 2 * rangeClosed(2, N).filter(LongUtils::isOdd).map(n -> 2 * pow(n, 2) - 3 * (n - 1)).sum();
    }

}
