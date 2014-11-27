/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import org.junit.Test;

import static org.junit.Assert.assertEquals;

/**
 * The prime factors of 13195 are 5, 7, 13 and 29.
 * What is the largest prime factor of the number 600851475143 ?
 *
 * @author barreiro
 */
public class Problem003Test extends ProjectEulerAbstractTest {

    @Test
    public void test() {
        assertEquals(6857, new Solver003().solve());
    }

    @Test
    public void example() {
        assertEquals(29, new Solver003(13195).solve());
    }

    @Test
    public void minimal() {
        assertEquals(3, new Solver003(12).solve());
    }

}
