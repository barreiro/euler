/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import org.junit.Test;

import static org.junit.Assert.assertEquals;

/**
 * Starting with the number 1 and moving to the right in a clockwise direction a 5 by 5 spiral is formed as follows:
 * <p>
 * 21 22 23 24 25
 * 20  7  8  9 10
 * 19  6  1  2 11
 * 18  5  4  3 12
 * 17 16 15 14 13
 * <p>
 * It can be verified that the sum of the numbers on the diagonals is 101.
 * <p>
 * What is the sum of the numbers on the diagonals in a 1001 by 1001 spiral formed in the same way?
 *
 * @author barreiro
 */
public class Problem028Test extends ProjectEulerAbstractTest {

    @Test
    public void test() {
        assertEquals(669171001, new Solver028().solve());
    }

    @Test
    public void example() {
        assertEquals(101, new Solver028(5).solve());
    }

    @Test
    public void minimal() {
        assertEquals(25, new Solver028(3).solve());
    }

}
