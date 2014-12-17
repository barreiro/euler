/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import org.junit.Test;

import static org.junit.Assert.assertEquals;

/**
 * In England the currency is made up of pound, £, and pence, p, and there are eight coins in general circulation:
 * <p>
 * 1p, 2p, 5p, 10p, 20p, 50p, £1 (100p) and £2 (200p).
 * <p>
 * It is possible to make £2 in the following way:
 * 1×£1 + 1×50p + 2×20p + 1×5p + 1×2p + 3×1p
 * <p>
 * How many different ways can £2 be made using any number of coins?
 *
 * @author barreiro
 */
public class Problem031Test extends ProjectEulerAbstractTest {

    @Test
    public void test() {
        assertEquals(73682, new Solver031().solve());
    }

    @Test
    public void minimal() {
        assertEquals(1, new Solver031(1).solve());
    }

    @Test
    public void verySmall1() {
        assertEquals(2, new Solver031(2).solve());
    }

    @Test
    public void verySmall2() {
        assertEquals(4, new Solver031(5).solve());
    }

    @Test
    public void verySmall3() {
        assertEquals(11, new Solver031(10).solve());
    }

    @Test
    public void big() {
        assertEquals(321335886, new Solver031(1000).solve());
    }

    @Test
    public void veryBig() {
        assertEquals(10082315214426L, new Solver031(5000).solve());
    }

}
