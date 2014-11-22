/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import org.junit.Assert;
import org.junit.Test;

/**
 * The four adjacent digits in the 1000-digit number that have the greatest product are 9 × 9 × 8 × 9 = 5832.
 * Find the thirteen adjacent digits in the 1000-digit number that have the greatest product. What is the value of this product?
 *
 * @author barreiro
 */
public class Problem008Test extends ProjectEulerAbstractTest {

    @Test
    public void test() {
        Assert.assertEquals(23514624000l, new Solver008().solve());
    }

    @Test
    public void example() {
        Assert.assertEquals(5832, new Solver008(4).solve());
    }

    @Test
    public void minimal() {
        Assert.assertEquals(8, new Solver008(1, "28").solve());
    }

    @Test
    public void big() {
        Assert.assertEquals(240789749760000l, new Solver008(20).solve());
    }

}
