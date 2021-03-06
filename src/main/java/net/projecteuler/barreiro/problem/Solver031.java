/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import java.util.Set;

import static java.util.Arrays.stream;
import static net.projecteuler.barreiro.algorithm.Combinatorics.partition;
import static net.projecteuler.barreiro.algorithm.util.StreamUtils.intSet;

/**
 * In England the currency is made up of pound, £, and pence, p, and there are eight coins in general circulation:
 * <p>
 * 1p, 2p, 5p, 10p, 20p, 50p, £1 (100p) and £2 (200p).
 * <p>
 * It is possible to make £2 in the following way:
 * 1×£1 + 1×50p + 2×20p + 1×5p + 1×2p + 3×1p
 * <p>
 * How many different ways can £2 be made using any number of coins?
 *
 * @author barreiro
 */
public class Solver031 extends ProjectEulerSolver {

    private static final Set<Integer> CURRENCY = intSet( stream( new int[]{1, 2, 5, 10, 20, 50, 100, 200} ) );

    public Solver031() {
        this( 200 );
    }

    public Solver031(long n) {
        super( n );
    }

    // --- //

    public long solve() {
        return partition( N, CURRENCY );
    }

}
