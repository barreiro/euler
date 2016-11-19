/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import org.junit.Test;

import static org.junit.Assert.assertEquals;

/**
 * The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
 * Find the sum of all the primes below two million.
 *
 * @author barreiro
 */
public class Problem010Test extends ProjectEulerAbstractTest {

    @Test
    public void test() {
        assertEquals( 142913828922L, new Solver010().solve() );
    }

    @Test
    public void example() {
        assertEquals( 17, new Solver010( 10 ).solve() );
    }

    @Test
    public void minimal() {
        assertEquals( 5, new Solver010( 5 ).solve() );
    }

}
