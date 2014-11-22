/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import org.junit.Assert;
import org.junit.Test;

/**
 * The prime factors of 13195 are 5, 7, 13 and 29.
 * What is the largest prime factor of the number 600851475143 ?
 *
 * @author barreiro
 */
public class Problem003Test extends ProjectEulerAbstractTest {

    @Test
    public void test() {
        Assert.assertEquals(6857, new Solver003().solve());
    }

    @Test
    public void example() {
        Assert.assertEquals(29, new Solver003(13195).solve());
    }

    @Test
    public void minimal() {
        Assert.assertEquals(3, new Solver003(12).solve());
    }

}
