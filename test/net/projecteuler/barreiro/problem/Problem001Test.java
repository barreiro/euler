/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import org.junit.Assert;
import org.junit.Test;

/**
 * If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
 * Find the sum of all the multiples of 3 or 5 below 1000.
 *
 * @author barreiro
 */
public class Problem001Test extends ProjectEulerAbstractTest {

    @Test
    public void test() {
        Assert.assertEquals(233168, new Solver001().solve());
    }

    @Test
    public void example() {
        Assert.assertEquals(23, new Solver001(10).solve());
    }

    @Test
    public void minimal() {
        Assert.assertEquals(8, new Solver001(6).solve());
    }

}
