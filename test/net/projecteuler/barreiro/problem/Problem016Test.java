/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import org.junit.Assert;
import org.junit.Test;

/**
 * 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
 * What is the sum of the digits of the number 2^1000?
 *
 * @author barreiro
 */
public class Problem016Test extends ProjectEulerAbstractTest {

    @Test
    public void test() {
        Assert.assertEquals(1366, new Solver016().solve());
    }

    @Test
    public void example() {
        Assert.assertEquals(26, new Solver016(15).solve());
    }

    @Test
    public void minimal0() {
        Assert.assertEquals(1, new Solver016(0).solve());
    }

    @Test
    public void minimal1() {
        Assert.assertEquals(2, new Solver016(1).solve());
    }

    @Test
    public void minimal2() {
        Assert.assertEquals(7, new Solver016(4).solve());
    }

    @Test
    public void minimal3() {
        Assert.assertEquals(13, new Solver016(8).solve());
    }

    @Test
    public void minimal4() {
        Assert.assertEquals(7, new Solver016(10).solve());
    }

    @Test
    public void minimal5() {
        Assert.assertEquals(62, new Solver016(33).solve());
    }

    @Test
    public void minimal6() {
        Assert.assertEquals(86, new Solver016(65).solve());
    }

    @Test
    public void minimal7() {
        Assert.assertEquals(115, new Solver016(100).solve());
    }

    @Test
    public void minimal8() {
        Assert.assertEquals(256, new Solver016(200).solve());
    }

    @Test
    public void minimal9() {
        Assert.assertEquals(679, new Solver016(500).solve());
    }

}
