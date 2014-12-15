/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import org.junit.Test;

import static org.junit.Assert.assertEquals;

/**
 * Surprisingly there are only three numbers that can be written as the sum of fourth powers of their digits:
 * <p>
 * 1634 = 1^4 + 6^4 + 3^4 + 4^4
 * 8208 = 8^4 + 2^4 + 0^4 + 8^4
 * 9474 = 9^4 + 4^4 + 7^4 + 4^4
 * As 1 = 1^4 is not a sum it is not included.
 * <p>
 * The sum of these numbers is 1634 + 8208 + 9474 = 19316.
 * <p>
 * Find the sum of all the numbers that can be written as the sum of fifth powers of their digits.
 *
 * @author barreiro
 */
public class Problem030Test extends ProjectEulerAbstractTest {

    @Test
    public void test() {
        assertEquals(443839, new Solver030().solve());
    }

    @Test
    public void example() {
        assertEquals(19316, new Solver030(4).solve());
    }

}
