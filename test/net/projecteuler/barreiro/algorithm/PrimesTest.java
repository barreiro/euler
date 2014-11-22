/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.algorithm;

import org.junit.Assert;
import org.junit.Test;

/**
 * @author barreiro
 */
public class PrimesTest {

    @Test
    public void primeGenerator() {
        Primes.Generator primeGenerator = new Primes.Generator();
        Assert.assertEquals(2, primeGenerator.nextPrime());
        Assert.assertEquals(3, primeGenerator.nextPrime());
        Assert.assertEquals(5, primeGenerator.nextPrime());
        Assert.assertEquals(7, primeGenerator.nextPrime());
        Assert.assertEquals(11, primeGenerator.nextPrime());
        Assert.assertEquals(13, primeGenerator.nextPrime());
        Assert.assertEquals(17, primeGenerator.nextPrime());
        Assert.assertEquals(19, primeGenerator.nextPrime());
        Assert.assertEquals(23, primeGenerator.nextPrime());
        Assert.assertEquals(29, primeGenerator.nextPrime());
        Assert.assertEquals(31, primeGenerator.nextPrime());
    }


}
