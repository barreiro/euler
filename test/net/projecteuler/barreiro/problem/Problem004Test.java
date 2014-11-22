/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import org.junit.Assert;
import org.junit.Test;

/**
 * A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
 * Find the largest palindrome made from the product of two 3-digit numbers.
 *
 * @author barreiro
 */
public class Problem004Test extends ProjectEulerAbstractTest {

    @Test
    public void test() {
        Assert.assertEquals(906609, new Solver004().solve());
    }

    @Test
    public void example() {
        Assert.assertEquals(9009, new Solver004(2).solve());
    }

    @Test
    public void minimal() {
        Assert.assertEquals(9, new Solver004(1).solve());
    }

    @Test
    public void big() {
        Assert.assertEquals(99000099, new Solver004(4).solve());
    }

    /* --- */

    @Test
    public void palindromic0() {
        Assert.assertEquals(false, Solver004.isPalindromic(15));
    }

    @Test
    public void palindromic1() {
        Assert.assertEquals(true, Solver004.isPalindromic(88));
    }

    @Test
    public void palindromic2() {
        Assert.assertEquals(false, Solver004.isPalindromic(15846));
    }

    @Test
    public void palindromic3() {
        Assert.assertEquals(true, Solver004.isPalindromic(84048));
    }
}
