/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import static java.util.stream.LongStream.range;
import static net.projecteuler.barreiro.algorithm.util.LongUtils.intSqrt;
import static net.projecteuler.barreiro.algorithm.util.StreamUtils.notZero;

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
        this( 1000 );
    }

    public Solver009(long n) {
        super( n );
    }

    // --- //

    // Solved with Euclides Formula --- a=m^2-n^2 --- b=2nm --- c=m^2+n^2 --- with m>n

    public long solve() {
        return notZero( range( 2, intSqrt( N ) ).map( m -> notZero( range( 1, m ).map( n -> {
            long a = m * m - n * n;
            long b = 2 * m * n;
            long c = m * m + n * n;
            
            return a + b + c == N ? a * b * c : 0;
        } ) ) ) );
    }

}
