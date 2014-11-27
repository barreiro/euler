/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.algorithm;

import org.junit.Test;

import static org.junit.Assert.assertEquals;

/**
 * @author barreiro
 */
public class PrimesTest {

    @Test
    public void primeGenerator() {
        Primes.Generator primeGenerator = new Primes.Generator();
        assertEquals(2, primeGenerator.nextPrime());
        assertEquals(3, primeGenerator.nextPrime());
        assertEquals(5, primeGenerator.nextPrime());
        assertEquals(7, primeGenerator.nextPrime());
        assertEquals(11, primeGenerator.nextPrime());
        assertEquals(13, primeGenerator.nextPrime());
        assertEquals(17, primeGenerator.nextPrime());
        assertEquals(19, primeGenerator.nextPrime());
        assertEquals(23, primeGenerator.nextPrime());
        assertEquals(29, primeGenerator.nextPrime());
        assertEquals(31, primeGenerator.nextPrime());
    }

}
