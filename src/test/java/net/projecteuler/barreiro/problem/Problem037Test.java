/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import org.junit.Test;

import static org.junit.Assert.assertEquals;

/**
 * The number 3797 has an interesting property. Being prime itself, it is possible to continuously remove digits from left to right, and remain prime at each stage: 3797, 797, 97, and 7. Similarly we can work from right to left: 3797, 379, 37, and 3.
 * <p>
 * Find the sum of the only eleven primes that are both truncatable from left to right and right to left.
 * <p>
 * NOTE: 2, 3, 5, and 7 are not considered to be truncatable primes.
 *
 * @author barreiro
 */
public class Problem037Test extends ProjectEulerAbstractTest {

    @Test
    public void test() {
        assertEquals( 748317, new Solver037().solve() );
    }

    @Test
    public void minimal() {
        assertEquals( 23, new Solver037( 1 ).solve() );
    }

    @Test
    public void small() {
        assertEquals( 186, new Solver037( 4 ).solve() );
    }

    @Test
    public void example() {
        assertEquals( 8920, new Solver037( 10 ).solve() );
    }

    @Test
    public void big() {
        assertEquals( 5123, new Solver037( 9 ).solve() );
    }

}
