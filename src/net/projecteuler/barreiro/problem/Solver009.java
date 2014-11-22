/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

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

    // Solved with Euclides Formula --- a=m^2-n^2 --- b=2nm --- c=m^2+n^2 --- with m>n

    /* --- */

    public long solve() {
        for (long m = 2; m < Math.sqrt(N); m++) {
            long mSquare = m * m;
            for (long n = 1; n < m; n++) {
                long nSquare = n * n;
                long a = mSquare - nSquare;
                long b = 2 * m * n;
                long c = mSquare + nSquare;

                if (a + b + c == N) {
                    return a * b * c;
                }
            }
        }
        return 0;
    }

}
