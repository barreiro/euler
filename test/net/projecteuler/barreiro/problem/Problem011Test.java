/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import org.junit.Test;

import static org.junit.Assert.assertEquals;

/**
 * In the 20×20 grid below, four numbers along a diagonal line have been marked in red.
 * The product of these numbers is 26 × 63 × 78 × 14 = 1788696.
 * <p/>
 * What is the greatest product of four adjacent numbers in the same direction (up, down, left, right, or diagonally) in the 20×20 grid?
 *
 * @author barreiro
 */
public class Problem011Test extends ProjectEulerAbstractTest {

    @Test
    public void test() {
        assertEquals(70600674, new Solver011().solve());
    }

    @Test
    public void example() {
        assertEquals(811502, new Solver011(3).solve());
    }

    @Test
    public void minimal() {
        assertEquals(99, new Solver011(1).solve());
    }

    @Test
    public void reduced() {
        assertEquals(9603, new Solver011(2).solve());
    }

}
