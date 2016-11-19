/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import org.junit.Test;

import static org.junit.Assert.assertEquals;

/**
 * We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n exactly once; for example, the 5-digit number, 15234, is 1 through 5 pandigital.
 *
 * The product 7254 is unusual, as the identity, 39 × 186 = 7254, containing multiplicand, multiplier, and product is 1 through 9 pandigital.
 *
 * Find the sum of all products whose multiplicand/multiplier/product identity can be written as a 1 through 9 pandigital.
 *
 * HINT: Some products can be obtained in more than one way so be sure to only include it once in your sum.
 *
 * @author barreiro
 */
public class Problem032Test extends ProjectEulerAbstractTest {

    @Test
    public void test() {
        assertEquals(45228, new Solver032().solve());
    }

    @Test
    public void minimal() {
        assertEquals(0, new Solver032(3).solve());
    }

    @Test
    public void small() {
        assertEquals(52, new Solver032(5).solve());
    }

    @Test
    public void medium1() {
        assertEquals(162, new Solver032(6).solve());
    }

    @Test
    public void medium2() {
        assertEquals(0, new Solver032(7).solve());
    }

    @Test
    public void medium3() {
        assertEquals(13458, new Solver032(8).solve());
    }

    @Test
    public void medium4() {
        assertEquals(45228, new Solver032(9).solve());
    }

}
