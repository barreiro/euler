/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import org.junit.Test;

import static org.junit.Assert.assertEquals;

/**
 * By starting at the top of the triangle below and moving to adjacent numbers on the row below, the maximum total from top to bottom is 23.
 * <pre style="text-align: center">
 * 3
 * 7 4
 * 2 4 6
 * 8 5 9 3</pre>
 * That is, 3 + 7 + 4 + 9 = 23.
 * Find the maximum total from top to bottom of the triangle below:
 * <p>
 * NOTE: As there are only 16384 routes, it is possible to solve this problem by trying every route.
 * However, Problem 67, is the same challenge with a triangle containing one-hundred rows; it cannot be solved by brute force, and requires a clever method! ;o)
 *
 * @author barreiro
 */
public class Problem018Test extends ProjectEulerAbstractTest {

    @Test
    public void test() {
        assertEquals(1074, new Solver018().solve());
    }

    @Test
    public void example() {
        assertEquals(23, new Solver018(4, "3\n7 4\n2 4 6\n8 5 9 3").solve());
    }

    @Test
    public void minimal() {
        assertEquals(75, new Solver018(1).solve());
    }

    @Test
    public void test1() {
        assertEquals(170, new Solver018(2).solve());
    }

}
