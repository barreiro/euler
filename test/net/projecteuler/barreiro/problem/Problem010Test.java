/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import org.junit.Assert;
import org.junit.Test;

/**
 * The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
 * Find the sum of all the primes below two million.
 *
 * @author barreiro
 */
public class Problem010Test extends ProjectEulerAbstractTest {

    @Test
    public void test() {
        Assert.assertEquals(142913828922l, new Solver010().solve());
    }

    @Test
    public void example() {
        Assert.assertEquals(17, new Solver010(10).solve());
    }

    @Test
    public void minimal() {
        Assert.assertEquals(5, new Solver010(5).solve());
    }

}
