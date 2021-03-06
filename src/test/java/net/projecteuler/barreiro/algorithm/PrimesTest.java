/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.algorithm;

import org.junit.Test;

import java.util.HashMap;
import java.util.Map;

import static net.projecteuler.barreiro.algorithm.Primes.millerRabin;
import static net.projecteuler.barreiro.algorithm.Primes.primeFactors;
import static net.projecteuler.barreiro.algorithm.Primes.primesLessThan;
import static net.projecteuler.barreiro.algorithm.Primes.primesStream;
import static net.projecteuler.barreiro.algorithm.Primes.primesStreamMillerRabin;
import static org.junit.Assert.assertArrayEquals;
import static org.junit.Assert.assertFalse;
import static org.junit.Assert.assertTrue;

/**
 * @author barreiro
 */
public class PrimesTest {

    @Test
    public void primeGenerator() {
        assertArrayEquals( new long[]{2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31}, primesStream().limit( 11 ).toArray() );
    }

    @Test
    public void primeGeneratorReverse() {
        assertArrayEquals( new long[]{31, 29, 23, 19, 17, 13, 11, 7, 5, 3, 2}, primesLessThan( 36 ).toArray() );
    }

    @Test
    public void generatorEquivalence() {
        assertArrayEquals( primesStreamMillerRabin().limit( 10000 ).toArray(), primesStream().limit( 10000 ).toArray() );
    }

    @Test
    public void millerRabinTest() {
        assertTrue( millerRabin( 2 ) && millerRabin( 3 ) && millerRabin( 5 ) && millerRabin( 7 ) && millerRabin( 11 ) && millerRabin( 13 ) );
        assertFalse( millerRabin( 4 ) || millerRabin( 6 ) || millerRabin( 8 ) || millerRabin( 9 ) || millerRabin( 10 ) || millerRabin( 12 ) );
    }

    @Test
    public void millerRabinCarmichael() {
        assertFalse( millerRabin( 561 ) || millerRabin( 1105 ) || millerRabin( 1729 ) || millerRabin( 2465 ) || millerRabin( 2821 ) || millerRabin( 6601 ) );
        assertFalse( millerRabin( 101101 ) || millerRabin( 252601 ) || millerRabin( 314821 ) || millerRabin( 340561 ) || millerRabin( 410041 ) || millerRabin( 512461 ) );
    }

    @Test
    public void millerRabinLong() {
        assertFalse( millerRabin( 154639673381L ) || millerRabin( 585226005592931977L ) || millerRabin( 7999252175582851L ) || millerRabin( 55245642489451L ) );
    }

    @Test
    public void millerRabinTestStream() {
        assertTrue( primesStream().limit( 100000 ).allMatch( Primes::millerRabin ) );
    }

    @Test
    public void primeFactorsTest() {
        Map<Long, Long> realFactors = new HashMap<>();
        realFactors.put( 3L, 1L );
        realFactors.put( 7L, 1L );

        Map<Long, Long> factors = primeFactors( 21L );

        assertTrue( factors.entrySet().size() == realFactors.entrySet().size() && factors.entrySet().containsAll( realFactors.entrySet() ) );
    }

}
