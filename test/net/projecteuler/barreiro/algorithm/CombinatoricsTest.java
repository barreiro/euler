/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.algorithm;

import org.junit.Test;

import static net.projecteuler.barreiro.algorithm.Combinatorics.choose;
import static org.junit.Assert.assertEquals;

/**
 * @author barreiro
 */
public class CombinatoricsTest {

    @Test
    public void minimal1() {
        assertEquals(1, choose(1, 1));
    }

    @Test
    public void minimal2() {
        assertEquals(2, choose(2, 1));
    }

    @Test
    public void minimal3() {
        assertEquals(0, choose(1, 2));
    }

    @Test
    public void minimal4() {
        assertEquals(10, choose(5, 3));
    }

    @Test
    public void error1() {
        assertEquals(0, choose(5, -3));
    }

    @Test
    public void error2() {
        assertEquals(0, choose(5, 13));
    }

    @Test
    public void test1() {
        assertEquals(2598960, choose(52, 5));
    }

    @Test
    public void test2() {
        assertEquals(choose(52, 5), choose(52, 52 - 5));
    }

    @Test
    public void big() {
        assertEquals(88004802264174740l, choose(60, 27));
    }

}
