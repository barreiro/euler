/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import org.junit.Test;

import static org.junit.Assert.assertEquals;

/**
 * A Pythagorean triplet is a set of three natural numbers, a < b < c, for which, a^2 + b^2 = c^2
 * For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.
 * <p/>
 * There exists exactly one Pythagorean triplet for which a + b + c = 1000.
 * Find the product abc.
 *
 * @author barreiro
 */
public class Problem009Test extends ProjectEulerAbstractTest {

    @Test
    public void test() {
        assertEquals( 31875000, new Solver009().solve() );
    }

    @Test
    public void example() {
        assertEquals( 60, new Solver009( 12 ).solve() );
    }

    @Test
    public void big() {
        assertEquals( 265387500000L, new Solver009( 20000 ).solve() );
    }

}
