/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.algorithm;

import org.junit.Assert;
import org.junit.Test;

/**
 * @author barreiro
 */
public class CombinatoricsTest {

    @Test
    public void minimal1() {
        Assert.assertEquals(1, Combinatorics.choose(1, 1));
    }

    @Test
    public void minimal2() {
        Assert.assertEquals(2, Combinatorics.choose(2, 1));
    }

    @Test
    public void minimal3() {
        Assert.assertEquals(0, Combinatorics.choose(1, 2));
    }

    @Test
    public void minimal4() {
        Assert.assertEquals(10, Combinatorics.choose(5, 3));
    }

    @Test
    public void test1() {
        Assert.assertEquals(2598960, Combinatorics.choose(52, 5));
    }

    @Test
    public void test2() {
        Assert.assertEquals(Combinatorics.choose(52, 5), Combinatorics.choose(52, 52 - 5));
    }

    @Test
    public void big() {
        Assert.assertEquals(88004802264174740l, Combinatorics.choose(60, 27));
    }

}
