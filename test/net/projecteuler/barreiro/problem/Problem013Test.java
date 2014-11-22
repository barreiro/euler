/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import org.junit.Assert;
import org.junit.Test;

/**
 * Work out the first ten digits of the sum of the following one-hundred 50-digit numbers.
 *
 * @author barreiro
 */
public class Problem013Test extends ProjectEulerAbstractTest {

    @Test
    public void test() {
        Assert.assertEquals(5537376230l, new Solver013().solve());
    }

    @Test
    public void big() {
        Assert.assertEquals(553737623039087l, new Solver013(15).solve());
    }

    @Test
    public void minimal1() {
        Assert.assertEquals(5, new Solver013(1).solve());
    }

    @Test
    public void minimal2() {
        Assert.assertEquals(55, new Solver013(2).solve());
    }

    @Test
    public void minimal3() {
        Assert.assertEquals(553, new Solver013(3).solve());
    }

    @Test
    public void minimal4() {
        Assert.assertEquals(5537, new Solver013(4).solve());
    }

    @Test
    public void minimal5() {
        Assert.assertEquals(55373, new Solver013(5).solve());
    }

}
