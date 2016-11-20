/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import org.junit.Test;

import static org.junit.Assert.assertEquals;

/**
 * The number, 197, is called a circular prime because all rotations of the digits: 197, 971, and 719, are themselves prime.
 * <p>
 * There are thirteen such primes below 100: 2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, and 97.
 * <p>
 * How many circular primes are there below one million?
 *
 * @author barreiro
 */
public class Problem035Test extends ProjectEulerAbstractTest {

    @Test
    public void test() {
        assertEquals( 55, new Solver035().solve() );
    }

    @Test
    public void minimal() {
        assertEquals( 2, new Solver035( 5 ).solve() );
    }

    @Test
    public void small() {
        assertEquals( 7, new Solver035( 20 ).solve() );
    }

    @Test
    public void example() {
        assertEquals( 13, new Solver035( 100 ).solve() );
    }

    @Test
    public void big() {
        assertEquals( 55, new Solver035( 5000000 ).solve() );
    }

}
