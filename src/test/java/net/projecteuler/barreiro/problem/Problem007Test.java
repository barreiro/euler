/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import org.junit.Test;

import static org.junit.Assert.assertEquals;

/**
 * By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
 * What is the 10001st prime number?
 *
 * @author barreiro
 */
public class Problem007Test extends ProjectEulerAbstractTest {

    @Test
    public void test() {
        assertEquals( 104743, new Solver007().solve() );
    }

    @Test
    public void example() {
        assertEquals( 13, new Solver007( 6 ).solve() );
    }

    @Test
    public void minimal() {
        assertEquals( 2, new Solver007( 1 ).solve() );
    }

    @Test
    public void big() {
        assertEquals( 1299709, new Solver007( 100000 ).solve() );
    }

}
