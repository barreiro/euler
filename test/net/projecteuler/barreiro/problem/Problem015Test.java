/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import org.junit.Assert;
import org.junit.Test;

/**
 * Starting in the top left corner of a 2x2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.
 * How many such routes are there through a 20x20 grid?
 *
 * @author barreiro
 */
public class Problem015Test extends ProjectEulerAbstractTest {

    @Test
    public void test() {
        Assert.assertEquals(137846528820l, new Solver015().solve());
    }

    @Test
    public void example() {
        Assert.assertEquals(6, new Solver015(2).solve());
    }

    @Test
    public void big() {
        Assert.assertEquals(118264581564861424l, new Solver015(30).solve());
    }

    @Test
    public void minimal1() {
        Assert.assertEquals(2, new Solver015(1).solve());
    }

    @Test
    public void minimal2() {
        Assert.assertEquals(20, new Solver015(3).solve());
    }

    @Test
    public void minimal3() {
        Assert.assertEquals(252, new Solver015(5).solve());
    }

    @Test
    public void minimal4() {
        Assert.assertEquals(184756, new Solver015(10).solve());
    }

}
