/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import org.junit.Test;

import static org.junit.Assert.assertEquals;

/**
 * The sequence of triangle numbers is generated by adding the natural numbers. So the 7th triangle number would be 1 + 2 + 3 + 4 + 5 + 6 + 7 = 28. The first ten terms would be:
 * 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
 * <p/>
 * Let us list the factors of the first seven triangle numbers:
 * <p/>
 * 1: 1
 * 3: 1,3
 * 6: 1,2,3,6
 * 10: 1,2,5,10
 * 15: 1,3,5,15
 * 21: 1,3,7,21
 * 28: 1,2,4,7,14,28
 * We can see that 28 is the first triangle number to have over five divisors.
 * <p/>
 * What is the value of the first triangle number to have over five hundred divisors?
 *
 * @author barreiro
 */
public class Problem012Test extends ProjectEulerAbstractTest {

    @Test
    public void test() {
        assertEquals(76576500, new Solver012().solve());
    }

    @Test
    public void example() {
        assertEquals(28, new Solver012(5).solve());
    }

    @Test
    public void minimal() {
        assertEquals(6, new Solver012(4).solve());
    }

}
