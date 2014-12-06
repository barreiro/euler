/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.algorithm;

import org.junit.Test;

import static net.projecteuler.barreiro.algorithm.Primes.*;
import static org.junit.Assert.*;

/**
 * @author barreiro
 */
public class PrimesTest {

    @Test
    public void primeGenerator() {
        assertArrayEquals(new long[]{2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31}, primesStream().limit(11).toArray());
    }

    @Test
    public void primeGeneratorReverse() {
        assertArrayEquals(new long[]{31, 29, 23, 19, 17, 13, 11, 7, 5, 3, 2}, primesLessThan(36).toArray());
    }

    @Test
    public void generatorEquivalence() {
        assertArrayEquals(primesStreamMillerRabin().limit(10000).toArray(), primesStream().limit(10000).toArray());
    }

    @Test
    public void millerRabinTest() {
        assertTrue(millerRabin(2) && millerRabin(3) && millerRabin(5) && millerRabin(7) && millerRabin(11) && millerRabin(13));
        assertFalse(millerRabin(4) && millerRabin(6) && millerRabin(8) && millerRabin(9) && millerRabin(10) && millerRabin(12));
    }

    @Test
    public void millerRabinTestStream() {
        assertTrue(primesStream().limit(10000).allMatch(Primes::millerRabin));
    }

}
