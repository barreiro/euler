/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import org.junit.Test;

import static org.junit.Assert.assertEquals;

/**
 * n! means n × (n − 1) × ... × 3 × 2 × 1
 * <p>
 * For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800, and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.
 * <p>
 * Find the sum of the digits in the number 100!
 *
 * @author barreiro
 */
public class Problem020Test extends ProjectEulerAbstractTest {

    @Test
    public void test() {
        assertEquals(648, new Solver020().solve());
    }

    @Test
    public void minimal1() {
        assertEquals(1, new Solver020(1).solve());
    }

    @Test
    public void minimal2() {
        assertEquals(2, new Solver020(2).solve());
    }

    @Test
    public void minimal3() {
        assertEquals(6, new Solver020(3).solve());
    }

    @Test
    public void minimal4() {
        assertEquals(6, new Solver020(4).solve());
    }

    @Test
    public void example() {
        assertEquals(27, new Solver020(10).solve());
    }

    @Test
    public void insane() {
        assertEquals(10539, new Solver020(1000).solve());
    }

}
