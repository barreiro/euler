/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import net.projecteuler.barreiro.algorithm.util.LongUtils;

import static java.lang.Math.sqrt;
import static java.util.stream.LongStream.range;

/**
 * A Pythagorean triplet is a set of three natural numbers, a < b < c, for which, a^2 + b^2 = c^2
 * For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.
 * <p/>
 * There exists exactly one Pythagorean triplet for which a + b + c = 1000.
 * Find the product abc.
 *
 * @author barreiro
 */
public class Solver009 extends ProjectEulerSolver {

    public Solver009() {
        this(1000);
    }

    public Solver009(long n) {
        super(n);
    }

    /* --- */

    // Solved with Euclides Formula --- a=m^2-n^2 --- b=2nm --- c=m^2+n^2 --- with m>n

    public long solve() {
        return range(2, (long) sqrt(N)).map(m -> range(1, m).map(n -> {
            long mSquare = m * m, nSquare = n * n, a = mSquare - nSquare, b = 2 * m * n, c = mSquare + nSquare;
            return a + b + c == N ? a * b * c : 0;
        }).filter(LongUtils::isntZero).findAny().orElse(0)).filter(LongUtils::isntZero).findAny().getAsLong();
    }

}
