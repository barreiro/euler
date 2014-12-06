/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import org.junit.Test;

import static org.junit.Assert.assertEquals;

/**
 * A unit fraction contains 1 in the numerator. The decimal representation of the unit fractions with denominators 2 to 10 are given:
 * <p>
 * 1/2	= 	0.5
 * 1/3	= 	0.(3)
 * 1/4	= 	0.25
 * 1/5	= 	0.2
 * 1/6	= 	0.1(6)
 * 1/7	= 	0.(142857)
 * 1/8	= 	0.125
 * 1/9	= 	0.(1)
 * 1/10	= 	0.1
 * <p>
 * Where 0.1(6) means 0.166666..., and has a 1-digit recurring cycle. It can be seen that 1/7 has a 6-digit recurring cycle.
 * <p>
 * Find the value of d < 1000 for which 1/d contains the longest recurring cycle in its decimal fraction part.
 *
 * @author barreiro
 */
public class Problem026Test extends ProjectEulerAbstractTest {

    @Test
    public void test() {
        assertEquals(983, new Solver026().solve());
    }

    @Test
    public void example() {
        assertEquals(7, new Solver026(10).solve());
    }

    @Test
    public void small() {
        assertEquals(97, new Solver026(100).solve());
    }

    @Test
    public void big() {
        assertEquals(9967, new Solver026(10000).solve());
    }

    @Test
    public void bigger() {
        assertEquals(99989, new Solver026(100000).solve());
    }

}
